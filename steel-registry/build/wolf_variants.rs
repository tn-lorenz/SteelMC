use std::fs;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use steel_utils::ResourceLocation;

#[derive(Deserialize, Debug)]
pub struct WolfVariantJson {
    assets: WolfAssetInfo,
    spawn_conditions: Vec<SpawnConditionEntry>,
}

#[derive(Deserialize, Debug)]
pub struct WolfAssetInfo {
    wild: ResourceLocation,
    tame: ResourceLocation,
    angry: ResourceLocation,
}

#[derive(Deserialize, Debug)]
pub struct SpawnConditionEntry {
    priority: i32,
    #[serde(default)]
    condition: Option<BiomeCondition>,
}

#[derive(Deserialize, Debug)]
pub struct BiomeCondition {
    #[serde(rename = "type")]
    condition_type: String,
    biomes: String,
}

fn generate_resource_location(resource: &ResourceLocation) -> TokenStream {
    let namespace = resource.namespace.as_ref();
    let path = resource.path.as_ref();
    quote! { ResourceLocation { namespace: Cow::Borrowed(#namespace), path: Cow::Borrowed(#path) } }
}

fn generate_option<T, F>(opt: &Option<T>, f: F) -> TokenStream
where
    F: FnOnce(&T) -> TokenStream,
{
    match opt {
        Some(val) => {
            let inner = f(val);
            quote! { Some(#inner) }
        }
        None => quote! { None },
    }
}

fn generate_biome_condition(condition: &BiomeCondition) -> TokenStream {
    let condition_type = condition.condition_type.as_str();
    let biomes = condition.biomes.as_str();

    quote! {
        BiomeCondition {
            condition_type: #condition_type,
            biomes: #biomes,
        }
    }
}

fn generate_spawn_condition_entry(entry: &SpawnConditionEntry) -> TokenStream {
    let priority = entry.priority;
    let condition = generate_option(&entry.condition, generate_biome_condition);

    quote! {
        SpawnConditionEntry {
            priority: #priority,
            condition: #condition,
        }
    }
}

pub(crate) fn build() -> TokenStream {
    println!(
        "cargo:rerun-if-changed=build_assets/builtin_datapacks/minecraft/data/minecraft/wolf_variant/"
    );

    let wolf_variant_dir = "build_assets/builtin_datapacks/minecraft/data/minecraft/wolf_variant";
    let mut wolf_variants = Vec::new();

    // Read all wolf variant JSON files
    for entry in fs::read_dir(wolf_variant_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let wolf_variant_name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path).unwrap();
            let wolf_variant: WolfVariantJson = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", wolf_variant_name, e));

            wolf_variants.push((wolf_variant_name, wolf_variant));
        }
    }

    // Sort wolf variants by name for consistent generation
    wolf_variants.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::wolf_variant::wolf_variant::{
            WolfVariant, WolfVariantRegistry, WolfAssetInfo, SpawnConditionEntry, BiomeCondition,
        };
        use steel_utils::ResourceLocation;
        use std::borrow::Cow;
    });

    // Generate static wolf variant definitions
    for (wolf_variant_name, wolf_variant) in &wolf_variants {
        let wolf_variant_ident =
            Ident::new(&wolf_variant_name.to_shouty_snake_case(), Span::call_site());
        let wolf_variant_name_str = wolf_variant_name.clone();

        let key = quote! { ResourceLocation::vanilla_static(#wolf_variant_name_str) };
        let wild = generate_resource_location(&wolf_variant.assets.wild);
        let tame = generate_resource_location(&wolf_variant.assets.tame);
        let angry = generate_resource_location(&wolf_variant.assets.angry);

        let spawn_conditions: Vec<_> = wolf_variant
            .spawn_conditions
            .iter()
            .map(generate_spawn_condition_entry)
            .collect();

        stream.extend(quote! {
            pub const #wolf_variant_ident: &WolfVariant = &WolfVariant {
                key: #key,
                assets: WolfAssetInfo {
                    wild: #wild,
                    tame: #tame,
                    angry: #angry,
                },
                spawn_conditions: &[#(#spawn_conditions),*],
            };
        });
    }

    // Generate registration function
    let mut register_stream = TokenStream::new();
    for (wolf_variant_name, _) in &wolf_variants {
        let wolf_variant_ident =
            Ident::new(&wolf_variant_name.to_shouty_snake_case(), Span::call_site());
        register_stream.extend(quote! {
            registry.register(&#wolf_variant_ident);
        });
    }

    stream.extend(quote! {
        pub fn register_wolf_variants(registry: &mut WolfVariantRegistry) {
            #register_stream
        }
    });

    stream
}
