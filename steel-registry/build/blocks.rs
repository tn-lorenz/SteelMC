use std::fs;

use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub id: u16,
    pub name: String,
    pub properties: Vec<String>,
    // example bool_true, int_5, enum_Direction_Down
    pub default_properties: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlockAssets {
    pub blocks: Vec<Block>,
    pub block_entity_types: Vec<String>,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=build_assets/blocks.json");
    let block_assets: BlockAssets =
        serde_json::from_str(&fs::read_to_string("build_assets/blocks.json").unwrap()).unwrap();

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::{
            behaviour::BlockBehaviourProperties,
            blocks::{Block, BlockRegistry},
            properties::BlockStateProperties,
        };
    });

    for block in &block_assets.blocks {
        let block_name = Ident::new(&block.name.to_shouty_snake_case(), Span::call_site());
        let block_name_str = block.name.clone();
        let properties = block
            .properties
            .iter()
            .map(|p| {
                let property_name = Ident::new(&p.to_shouty_snake_case(), Span::call_site());
                quote! {
                    &BlockStateProperties::#property_name
                }
            })
            .collect::<Vec<_>>();

        // Generate default state if block has properties
        let default_state = if !block.properties.is_empty() && !block.default_properties.is_empty()
        {
            let property_values = block
                .properties
                .iter()
                .zip(block.default_properties.iter())
                .map(|(prop_name, default_val)| {
                    let property_ident =
                        Ident::new(&prop_name.to_shouty_snake_case(), Span::call_site());

                    // Parse the default value format
                    let value_expr = if default_val.starts_with("bool_") {
                        // Boolean: "bool_true" or "bool_false"
                        let bool_val = default_val == "bool_true";
                        quote! {
                            BlockStateProperties::#property_ident.index_of(#bool_val)
                        }
                    } else if default_val.starts_with("int_") {
                        // Integer: "int_5"
                        let int_val = default_val
                            .strip_prefix("int_")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();
                        quote! { #int_val }
                    } else if default_val.starts_with("enum_") {
                        // Enum: "enum_Direction_Down" -> Direction::Down
                        let enum_part = default_val.strip_prefix("enum_").unwrap();
                        let parts: Vec<&str> = enum_part.split('_').collect();

                        if parts.len() >= 2 {
                            // First part is enum type, rest is variant name
                            let enum_type = parts[0];
                            let variant_name = parts[1..].join("_");

                            println!("enum_type: {}, variant_name: {}", enum_type, variant_name);
                            let enum_type_ident = Ident::new(enum_type, Span::call_site());
                            let variant_ident =
                                Ident::new(&variant_name.to_upper_camel_case(), Span::call_site());

                            quote! { BlockStateProperties::#property_ident.get_internal_index_const(&crate::properties::#enum_type_ident::#variant_ident) }
                        } else {
                            // Fallback if format is unexpected
                            quote! { 0 }
                        }
                    } else {
                        // Unknown format, default to 0
                        quote! { 0 }
                    };

                    quote! {
                        BlockStateProperties::#property_ident => #value_expr
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                .with_default_state(crate::blocks::offset!(
                    #(#property_values),*
                ))
            }
        } else {
            quote! {}
        };

        stream.extend(quote! {
            pub const #block_name: Block = Block::new(
                #block_name_str,
                BlockBehaviourProperties::new(),
                &[
                    #(#properties),*
                ],
            )#default_state;
        });
    }

    let mut register_stream = TokenStream::new();
    for block in &block_assets.blocks {
        let block_name = Ident::new(&block.name.to_shouty_snake_case(), Span::call_site());
        register_stream.extend(quote! {
            registry.register(&#block_name);
        });
    }

    stream.extend(quote! {
        pub fn register_blocks(registry: &mut BlockRegistry) {
            #register_stream
        }
    });

    stream
}
