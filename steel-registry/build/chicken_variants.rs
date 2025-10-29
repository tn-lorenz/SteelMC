use std::fs;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use steel_utils::ResourceLocation;

#[derive(Deserialize, Debug)]
pub struct ChickenVariantJson {
    asset_id: ResourceLocation,
    #[serde(default)]
    model: String,
    spawn_conditions: Vec<SpawnConditionEntry>,
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

fn generate_chicken_model_type(model: &str) -> TokenStream {
    match model {
        "cold" => quote! { ChickenModelType::Cold },
        _ => quote! { ChickenModelType::Normal },
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
        "cargo:rerun-if-changed=build_assets/builtin_datapacks/minecraft/data/minecraft/chicken_variant/"
    );

    let chicken_variant_dir =
        "build_assets/builtin_datapacks/minecraft/data/minecraft/chicken_variant";
    let mut chicken_variants = Vec::new();

    // Read all chicken variant JSON files
    for entry in fs::read_dir(chicken_variant_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let chicken_variant_name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path).unwrap();
            let chicken_variant: ChickenVariantJson = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", chicken_variant_name, e));

            chicken_variants.push((chicken_variant_name, chicken_variant));
        }
    }

    // Sort chicken variants by name for consistent generation
    chicken_variants.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::chicken_variant::chicken_variant::{
            ChickenVariant, ChickenVariantRegistry, ChickenModelType, SpawnConditionEntry, BiomeCondition,
        };
        use steel_utils::ResourceLocation;
        use std::borrow::Cow;
    });

    // Generate static chicken variant definitions
    for (chicken_variant_name, chicken_variant) in &chicken_variants {
        let chicken_variant_ident = Ident::new(
            &chicken_variant_name.to_shouty_snake_case(),
            Span::call_site(),
        );
        let chicken_variant_name_str = chicken_variant_name.clone();

        let key = quote! { ResourceLocation::vanilla_static(#chicken_variant_name_str) };
        let asset_id = generate_resource_location(&chicken_variant.asset_id);
        let model = generate_chicken_model_type(&chicken_variant.model);

        let spawn_conditions: Vec<_> = chicken_variant
            .spawn_conditions
            .iter()
            .map(generate_spawn_condition_entry)
            .collect();

        stream.extend(quote! {
            pub const #chicken_variant_ident: &ChickenVariant = &ChickenVariant {
                key: #key,
                asset_id: #asset_id,
                model: #model,
                spawn_conditions: &[#(#spawn_conditions),*],
            };
        });
    }

    // Generate registration function
    let mut register_stream = TokenStream::new();
    for (chicken_variant_name, _) in &chicken_variants {
        let chicken_variant_ident = Ident::new(
            &chicken_variant_name.to_shouty_snake_case(),
            Span::call_site(),
        );
        register_stream.extend(quote! {
            registry.register(&#chicken_variant_ident);
        });
    }

    stream.extend(quote! {
        pub fn register_chicken_variants(registry: &mut ChickenVariantRegistry) {
            #register_stream
        }
    });

    stream
}
