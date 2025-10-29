use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Meta, parse_macro_input};

const ALLOWED_TYPES: [&str; 12] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

const UNSUPPORTED_PROP: &str =
    "unsupported property. Supported properties are `as = \"...\"`, `bound = ...`, `prefix = ...`";
const WRONG_FORMAT: &str =
    "attribute requires a list format: `#[read_as(as = \"...\", bound = ..., ..)]";

#[proc_macro_derive(PacketRead, attributes(read_as))]
pub fn packet_read_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        Data::Struct(s) => {
            let Fields::Named(fields) = s.fields else {
                panic!("PacketRead only supports structs with named fields");
            };

            // Create read calls for every field
            let readers = fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = &f.ty;

                let mut read_strategy: Option<String> = None;
                let mut bound: Option<syn::LitInt> = None;
                let mut prefix: Option<syn::Type> = None;

                if let Some(attr) = f.attrs.iter().find(|a| a.path().is_ident("read_as")) {
                    if let Meta::List(meta) = attr.meta.clone() {
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("as") {
                                let value = meta.value()?;
                                let s: LitStr = value.parse()?;
                                read_strategy = Some(s.value());
                                Ok(())
                            } else if meta.path.is_ident("bound") {
                                let value = meta.value()?;
                                let int_lit: syn::LitInt = value.parse()?;
                                bound = Some(int_lit);
                                Ok(())
                            } else if meta.path.is_ident("prefix") {
                                let value = meta.value()?;
                                let type_lit: syn::Type = value.parse()?;
                                prefix = Some(type_lit);
                                Ok(())
                            } else {
                                Err(meta.error(UNSUPPORTED_PROP))
                            }
                        })
                        .unwrap_or_else(|e| panic!("Failed to parse `read_as` attribute: {}", e));
                    } else {
                        panic!("{WRONG_FORMAT}");
                    }
                }

                match read_strategy.as_deref() {
                    Some("var_int") => quote! {
                        let #field_name = crate::codec::VarInt::read(data)?.0 as #field_type;
                    },
                    Some("string") | Some("vec") => {
                        let prefix =
                            prefix.unwrap_or_else(|| syn::parse_quote!(crate::codec::VarInt));

                        let read_call = if let Some(b) = bound {
                            quote! { <#field_type>::read_prefixed_bound::<#prefix>(data, #b)? }
                        } else {
                            quote! { <#field_type>::read_prefixed::<#prefix>(data)? }
                        };

                        quote! {
                            let #field_name = #read_call;
                        }
                    }
                    None => quote! {
                        let #field_name = <#field_type>::read(data)?;
                    },
                    Some(s) => panic!("Unknown read strategy: `{}`", s),
                }
            });

            let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            let expanded = quote! {
                #[automatically_derived]
                impl crate::packet_traits::ReadFrom for #name {
                    fn read(data: &mut impl std::io::Read) -> std::io::Result<Self>{
                        use crate::packet_traits::PrefixedRead;

                        #(#readers)*

                        Ok(Self {
                            #(#field_names),*
                        })
                    }
                }

                #[automatically_derived]
                impl crate::packet_traits::PacketRead for #name {}
            };

            TokenStream::from(expanded)
        }
        Data::Enum(e) => {
            let readers = e.variants.iter().map(|v| {
                if !matches!(v.fields, Fields::Unit) {
                    panic!("PacketReader only supports enum variants without fields");
                }
                let Some((_, value)) = &v.discriminant else {
                    panic!("PacketReader only supports enum variants with explicit discriminant\n(Ej. {} = 0)", &v.ident)
                };
                let v_name = &v.ident;
                quote! {
                    #value => #name::#v_name,
                }
            });

            // Support reading the enum discriminant using a specified strategy
            // Defaults to reading a varint when no attribute is provided
            let mut read_strategy: Option<String> = None;
            let mut bound: Option<syn::LitInt> = None;
            if let Some(attr) = input.attrs.iter().find(|a| a.path().is_ident("read_as")) {
                if let Meta::List(meta) = attr.meta.clone() {
                    meta.parse_nested_meta(|meta| {
                        if meta.path.is_ident("as") {
                            let value = meta.value()?;
                            let s: LitStr = value.parse()?;
                            read_strategy = Some(s.value());
                            Ok(())
                        } else if meta.path.is_ident("bound") {
                            let value = meta.value()?;
                            let int_lit: syn::LitInt = value.parse()?;
                            bound = Some(int_lit);
                            Ok(())
                        } else {
                            Err(meta.error(UNSUPPORTED_PROP))
                        }
                    })
                    .unwrap_or_else(|e| panic!("Failed to parse `read_as` attribute: {}", e));
                } else {
                    panic!("{WRONG_FORMAT}");
                }
            }

            let read_discriminant = match read_strategy.as_deref() {
                // Specialized implementation: read a VarInt (i32)
                None | Some("var_int") => quote! { crate::codec::VarInt::read(data)?.into() },
                // Simple implementation: read as a primitive numeric type
                Some(s) => {
                    if !ALLOWED_TYPES.contains(&s) {
                        panic!("Unknown read strategy: `{}`", s)
                    }
                    let enum_type = Ident::new(s, Span::call_site());
                    let _ = bound; // `bound` currently unused for primitive reads
                    quote! { <#enum_type as PacketRead>::read_packet(data)? }
                }
            };

            let error_msg = format!("Invalid {name}");

            TokenStream::from(quote! {
                #[automatically_derived]
                impl crate::packet_traits::ReadFrom for #name {
                    fn read(data: &mut impl std::io::Read) -> std::io::Result<Self> {
                        Ok(match { #read_discriminant } {
                            #(#readers)*
                            _ => {
                                return Err(
                                    std::io::Error::other(#error_msg)
                                );
                            }
                        })
                    }
                }

                #[automatically_derived]
                impl crate::packet_traits::PacketRead for #name {}
            })
        }
        _ => panic!("PacketRead can only be derived for structs or enums"),
    }
}

#[proc_macro_derive(PacketWrite, attributes(write_as))]
pub fn packet_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        Data::Struct(s) => {
            let Fields::Named(fields) = s.fields else {
                panic!("PacketWrite only supports structs with named fields");
            };

            let mut json_count = 0;

            let writers = fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();

                let mut write_strategy: Option<String> = None;
                let mut bound: Option<syn::LitInt> = None;
                let mut prefix: Option<syn::Type> = None;

                if let Some(attr) = f.attrs.iter().find(|a| a.path().is_ident("write_as")) {
                    if let Meta::List(meta) = attr.meta.clone() {
                        meta.parse_nested_meta(|meta| {
                            if meta.path.is_ident("as") {
                                let value = meta.value()?;
                                let s: LitStr = value.parse()?;
                                write_strategy = Some(s.value());
                                Ok(())
                            } else if meta.path.is_ident("bound") {
                                let value = meta.value()?;
                                let int_lit: syn::LitInt = value.parse()?;
                                bound = Some(int_lit);
                                Ok(())
                            } else if meta.path.is_ident("prefix") {
                                let value = meta.value()?;
                                let type_lit: syn::Type = value.parse()?;
                                prefix = Some(type_lit);
                                Ok(())
                            } else {
                                Err(meta.error(UNSUPPORTED_PROP))
                            }
                        })
                        .unwrap_or_else(|e| panic!("Failed to parse `write_as` attribute: {}", e));
                    } else {
                        panic!("{WRONG_FORMAT}");
                    }
                }

                match write_strategy.as_deref() {
                    Some("var_int") => quote! {
                        crate::codec::VarInt(self.#field_name).write(writer)?;
                    },
                    Some("string") | Some("vec") => {
                        let prefix = prefix.unwrap_or_else(|| syn::parse_quote!(crate::codec::VarInt));

                        let write_call = if let Some(b) = bound {
                            quote! { self.#field_name.write_prefixed_bound::<#prefix>(writer, #b)?; }
                        } else {
                            quote! { self.#field_name.write_prefixed::<#prefix>(writer)?; }
                        };

                        quote! {
                            {
                                use crate::packet_traits::PrefixedWrite;
                                #write_call
                            }
                        }
                    },
                    Some("json") => {
                        json_count += 1;
                        let prefix = prefix.unwrap_or_else(|| syn::parse_quote!(crate::codec::VarInt));
                        quote! {
                            use crate::packet_traits::PrefixedWrite;

                            serde_json::to_string(&self.#field_name).map_err(|e| {
                                std::io::Error::other(format!("Failed to serialize: {e}"))
                            })?.write_prefixed::<#prefix>(writer)?;
                        }
                    },
                    Some("option") => {
                        quote! {
                            if let Some(value) = &self.#field_name {
                                true.write(writer)?;
                                crate::packet_traits::WriteTo::write(value, writer)?;
                            } else {
                                false.write(writer)?;
                            }
                        }
                    },
                    None => quote! {
                        self.#field_name.write(writer)?;
                    },
                    Some(s) => panic!("Unknown write strategy: `{s} {json_count}`"),
                }
            });

            let expanded = quote! {
                #[automatically_derived]
                impl crate::packet_traits::WriteTo for #name {
                    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                        #(#writers)*

                        Ok(())
                    }
                }

                #[automatically_derived]
                impl crate::packet_traits::PacketWrite for #name {}
            };

            TokenStream::from(expanded)
        }
        Data::Enum(_) => {
            let mut write_strategy: Option<String> = None;
            let mut bound: Option<syn::LitInt> = None;
            if let Some(attr) = input.attrs.iter().find(|a| a.path().is_ident("write_as")) {
                if let Meta::List(meta) = attr.meta.clone() {
                    meta.parse_nested_meta(|meta| {
                        if meta.path.is_ident("as") {
                            let value = meta.value()?;
                            let s: LitStr = value.parse()?;
                            write_strategy = Some(s.value());
                            Ok(())
                        } else if meta.path.is_ident("bound") {
                            let value = meta.value()?;
                            let int_lit: syn::LitInt = value.parse()?;
                            bound = Some(int_lit);
                            Ok(())
                        } else {
                            Err(meta.error(UNSUPPORTED_PROP))
                        }
                    })
                    .unwrap_or_else(|e| panic!("Failed to parse `write_as` attribute: {}", e));
                } else {
                    panic!("{WRONG_FORMAT}");
                }
            } else {
                panic!("PacketWrite enums requires the \"write_as\" attribute")
            }

            let writer = match write_strategy.as_deref() {
                // Specialiced implementation
                Some("var_int") => {
                    quote! {
                        crate::codec::VarInt(*self as i32).write(writer)?;
                    }
                }
                // Simple implementation
                Some(s) => {
                    if !ALLOWED_TYPES.contains(&s) {
                        panic!("Unknown write strategy: `{}`", s)
                    }
                    let enum_type = Ident::new(s, Span::call_site());
                    quote! {
                        (*self as #enum_type).write_packet(writer)?;
                    }
                }
                None => panic!("Expected write_as's \"as\" value"),
            };
            TokenStream::from(quote! {
                #[automatically_derived]
                impl crate::packet_traits::WriteTo for #name {
                    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                        #writer

                        Ok(())
                    }
                }

                #[automatically_derived]
                impl crate::packet_traits::PacketWrite for #name {}
            })
        }
        _ => panic!("PacketWrite can only be derived for structs and enums"),
    }
}

#[proc_macro_derive(CBoundPacket, attributes(packet_id))]
pub fn cbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Find the packet_id attributes
    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("packet_id"))
        .collect();

    if attrs.is_empty() {
        panic!("CBoundPacket derive macro requires at least one #[packet_id(...)] attribute");
    }

    let mut match_arms = Vec::new();

    for attr in attrs {
        if let Meta::List(meta) = attr.meta.clone() {
            meta.parse_nested_meta(|meta| {
                let state = meta
                    .path
                    .get_ident()
                    .expect("Expected an identifier for the protocol state")
                    .to_string();
                let value = meta.value()?;
                let id_path: LitStr = value.parse()?;

                let state_ident = Ident::new(&state, Span::call_site());
                let id_path_val = id_path.value();
                let id_ident: syn::Path =
                    syn::parse_str(&id_path_val).expect("Failed to parse id path");

                let arm = quote! {
                    crate::utils::ConnectionProtocol::#state_ident => Some(#id_ident as i32),
                };
                match_arms.push(arm);

                Ok(())
            })
            .unwrap_or_else(|e| panic!("Failed to parse `packet_id` attribute: {}", e));
        } else {
            panic!("`packet_id` attribute must be a list: `#[packet_id(STATE = \"path\", ...)]`");
        }
    }

    let expanded = quote! {
        #[automatically_derived]
        impl crate::packet_traits::CBoundPacket for #name {
            fn get_id(&self, protocol: crate::utils::ConnectionProtocol) -> Option<i32> {
                match protocol {
                    #(#match_arms)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
