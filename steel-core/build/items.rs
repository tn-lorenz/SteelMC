//! Code generation for item behaviors.

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItemClass {
    pub name: String,
    pub class: String,
    #[serde(default)]
    pub block: Option<String>,
}

/// Items use lowercase field names (`vanilla_items::ITEMS.stone`)
fn to_item_field(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

/// Blocks use `SCREAMING_SNAKE_CASE` constants (`vanilla_blocks::STONE`)
fn to_block_const(name: &str) -> Ident {
    Ident::new(&name.to_shouty_snake_case(), Span::call_site())
}

fn generate_block_item_registrations<'a>(
    items: impl Iterator<Item = &'a (Ident, Ident)>,
) -> TokenStream {
    let registrations = items.map(|(item_field, block_const)| {
        quote! {
            registry.set_behavior(
                &vanilla_items::ITEMS.#item_field,
                Box::new(BlockItemBehavior::new(vanilla_blocks::#block_const)),
            );
        }
    });
    quote! { #(#registrations)* }
}

fn generate_sign_item_registrations<'a>(
    items: impl Iterator<Item = &'a (Ident, Ident, Ident)>,
) -> TokenStream {
    let registrations = items.map(|(item_field, standing_const, wall_const)| {
        quote! {
            registry.set_behavior(
                &vanilla_items::ITEMS.#item_field,
                Box::new(SignItemBehavior::new(vanilla_blocks::#standing_const, vanilla_blocks::#wall_const)),
            );
        }
    });
    quote! { #(#registrations)* }
}

fn generate_hanging_sign_item_registrations<'a>(
    items: impl Iterator<Item = &'a (Ident, Ident, Ident)>,
) -> TokenStream {
    let registrations = items.map(|(item_field, ceiling_const, wall_const)| {
        quote! {
            registry.set_behavior(
                &vanilla_items::ITEMS.#item_field,
                Box::new(HangingSignItemBehavior::new(vanilla_blocks::#ceiling_const, vanilla_blocks::#wall_const)),
            );
        }
    });
    quote! { #(#registrations)* }
}

fn generate_standing_and_wall_item_registrations<'a>(
    items: impl Iterator<Item = &'a (Ident, Ident, Ident)>,
) -> TokenStream {
    let registrations = items.map(|(item_field, standing_const, wall_const)| {
        quote! {
            registry.set_behavior(
                &vanilla_items::ITEMS.#item_field,
                Box::new(StandingAndWallBlockItem::new(vanilla_blocks::#standing_const, vanilla_blocks::#wall_const)),
            );
        }
    });
    quote! { #(#registrations)* }
}

fn generate_simple_registrations<'a>(
    items: impl Iterator<Item = &'a Ident>,
    behavior_type: &Ident,
) -> TokenStream {
    let registrations = items.map(|item_field| {
        quote! {
            registry.set_behavior(
                &vanilla_items::ITEMS.#item_field,
                Box::new(#behavior_type),
            );
        }
    });
    quote! { #(#registrations)* }
}

/// Derives wall sign block name from standing sign block name.
/// e.g., "`oak_sign`" -> "`oak_wall_sign`"
fn derive_wall_sign_block(standing_block: &str) -> String {
    // standing_block is like "oak_sign", we need "oak_wall_sign"
    if let Some(prefix) = standing_block.strip_suffix("_sign") {
        format!("{prefix}_wall_sign")
    } else {
        // Fallback, shouldn't happen with valid data
        format!("{standing_block}_wall")
    }
}

/// Derives wall hanging sign block name from ceiling hanging sign block name.
/// e.g., "`oak_hanging_sign`" -> "`oak_wall_hanging_sign`"
fn derive_wall_hanging_sign_block(ceiling_block: &str) -> String {
    // ceiling_block is like "oak_hanging_sign", we need "oak_wall_hanging_sign"
    if let Some(prefix) = ceiling_block.strip_suffix("_hanging_sign") {
        format!("{prefix}_wall_hanging_sign")
    } else {
        // Fallback, shouldn't happen with valid data
        format!("{ceiling_block}_wall")
    }
}

/// Derives wall block name from standing block name for torches, coral fans, skulls, heads, etc.
/// e.g., "`torch`" -> "`wall_torch`", "`soul_torch`" -> "`soul_wall_torch`"
/// e.g., "`tube_coral_fan`" -> "`tube_coral_wall_fan`"
/// e.g., "`skeleton_skull`" -> "`skeleton_wall_skull`"
/// e.g., "`zombie_head`" -> "`zombie_wall_head`"
fn derive_wall_block(standing_block: &str) -> String {
    // For torches: "torch" -> "wall_torch", "soul_torch" -> "soul_wall_torch"
    if standing_block == "torch" {
        return "wall_torch".to_string();
    }
    if let Some(prefix) = standing_block.strip_suffix("_torch") {
        return format!("{prefix}_wall_torch");
    }

    // For coral fans: "tube_coral_fan" -> "tube_coral_wall_fan"
    // Pattern: *_coral_fan -> *_coral_wall_fan
    if let Some(prefix) = standing_block.strip_suffix("_coral_fan") {
        return format!("{prefix}_coral_wall_fan");
    }

    // For skulls: "skeleton_skull" -> "skeleton_wall_skull"
    // Pattern: *_skull -> *_wall_skull
    if let Some(prefix) = standing_block.strip_suffix("_skull") {
        return format!("{prefix}_wall_skull");
    }

    // For heads: "zombie_head" -> "zombie_wall_head"
    // Pattern: *_head -> *_wall_head
    if let Some(prefix) = standing_block.strip_suffix("_head") {
        return format!("{prefix}_wall_head");
    }

    // Generic fallback: just prepend "wall_"
    format!("wall_{standing_block}")
}

pub fn build(items: &[ItemClass]) -> String {
    let mut block_items: Vec<(Ident, Ident)> = Vec::new();
    let mut sign_items: Vec<(Ident, Ident, Ident)> = Vec::new();
    let mut hanging_sign_items: Vec<(Ident, Ident, Ident)> = Vec::new();
    let mut standing_and_wall_items: Vec<(Ident, Ident, Ident)> = Vec::new();
    let mut ender_eye_items: Vec<Ident> = Vec::new();

    for item in items {
        let item_field = to_item_field(&item.name);

        match item.class.as_str() {
            "BlockItem" | "DoubleHighBlockItem" => {
                if let Some(block) = &item.block {
                    block_items.push((item_field, to_block_const(block)));
                }
            }
            "SignItem" => {
                if let Some(block) = &item.block {
                    let standing_const = to_block_const(block);
                    let wall_block = derive_wall_sign_block(block);
                    let wall_const = to_block_const(&wall_block);
                    sign_items.push((item_field, standing_const, wall_const));
                }
            }
            "HangingSignItem" => {
                if let Some(block) = &item.block {
                    let ceiling_const = to_block_const(block);
                    let wall_block = derive_wall_hanging_sign_block(block);
                    let wall_const = to_block_const(&wall_block);
                    hanging_sign_items.push((item_field, ceiling_const, wall_const));
                }
            }
            "StandingAndWallBlockItem" => {
                if let Some(block) = &item.block {
                    let standing_const = to_block_const(block);
                    let wall_block = derive_wall_block(block);
                    let wall_const = to_block_const(&wall_block);
                    standing_and_wall_items.push((item_field, standing_const, wall_const));
                }
            }
            "EnderEyeItem" => ender_eye_items.push(item_field),
            _ => {}
        }
    }

    let block_item_registrations = generate_block_item_registrations(block_items.iter());
    let sign_item_registrations = generate_sign_item_registrations(sign_items.iter());
    let hanging_sign_item_registrations =
        generate_hanging_sign_item_registrations(hanging_sign_items.iter());
    let standing_and_wall_item_registrations =
        generate_standing_and_wall_item_registrations(standing_and_wall_items.iter());

    let ender_eye_type = Ident::new("EnderEyeBehavior", Span::call_site());
    let ender_eye_registrations =
        generate_simple_registrations(ender_eye_items.iter(), &ender_eye_type);

    let output = quote! {
        //! Generated item behavior assignments.

        use steel_registry::{vanilla_blocks, vanilla_items};
        use crate::behavior::ItemBehaviorRegistry;
        use crate::behavior::items::{BlockItemBehavior, EnderEyeBehavior, HangingSignItemBehavior, SignItemBehavior, StandingAndWallBlockItem};

        pub fn register_item_behaviors(registry: &mut ItemBehaviorRegistry) {
            #block_item_registrations
            #sign_item_registrations
            #hanging_sign_item_registrations
            #standing_and_wall_item_registrations
            #ender_eye_registrations
        }
    };

    output.to_string()
}
