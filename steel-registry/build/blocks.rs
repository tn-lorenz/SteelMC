use std::fs;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use steel_utils::math::vector3::Vector3;

#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub id: u16,
    pub name: String,
    pub properties: Vec<String>,
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
        stream.extend(quote! {
            pub const #block_name: Block = Block::new(
                #block_name_str,
                BlockBehaviourProperties::new(),
                &[
                    #(#properties),*
                ],
            );
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
