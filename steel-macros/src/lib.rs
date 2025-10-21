use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, Meta, parse_macro_input};

#[proc_macro_derive(PacketRead, attributes(read_as))]
pub fn packet_read_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("PacketRead can only be derived for structs"),
    };
    let Fields::Named(fields) = fields else {
        panic!("PacketRead only supports structs with named fields");
    };

    let readers = fields.named.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;
        let mut read_strategy: Option<String> = None;
        let mut bound: Option<syn::LitInt> = None;

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
                    } else {
                        Err(meta.error(
                            "unsupported property in `read_as` attribute. Expected `as = \"...\"` or `bound = ...`",
                        ))
                    }
                })
                .unwrap_or_else(|e| panic!("Failed to parse `read_as` attribute: {}", e));
            } else {
                panic!("`read_as` attribute requires a list format: `#[read_as(as = \"...\", bound = ...)]`");
            }
        }

        match read_strategy.as_deref() {
            // FIX 2: Use `data.reader()` for var_int, just like the manual impl.
            // We also add `use bytes::Buf;` to ensure the `.reader()` method is available.
            Some("var_int") => quote! {
                let #field_name = {
                    use bytes::Buf;
                    var_int::read(&mut data.reader())?
                };
            },
            Some("string") => {
                let read_call = if let Some(b) = bound {
                    quote! { data.reader().get_string_bounded(#b)? }
                } else {
                    quote! { data.reader().get_string()? }
                };

                quote! {
                    let #field_name = {
                        use crate::ser::NetworkReadExt;
                        #read_call
                    };
                }
            }
            // This case was already correct.
            None => quote! {
                let #field_name = <#field_type>::read_packet(data)?;
            },
            Some(s) => panic!("Unknown read strategy: `{}`", s),
        }
    });

    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

    let expanded = quote! {
        // The trait implementation for the struct.
        impl PacketRead for #name {
            // FIX 3: Signature now takes `&mut bytes::Bytes` to match the trait.
            // FIX 1: The return type uses a fully qualified path, no `use` keyword.
            fn read_packet(data: &mut bytes::Bytes) -> Result<Self, crate::utils::PacketReadError>
            where
                Self: Sized,
            {
                // Execute all generated field readers to create local variables
                #(#readers)*

                // Construct the struct from the read fields
                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
