//! Build script for generating vanilla loot table definitions.

use std::{fs, path::Path};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct LootTableJson {
    #[serde(rename = "type")]
    loot_type: Option<String>,
    #[serde(default)]
    pools: Vec<LootPoolJson>,
    #[serde(default)]
    random_sequence: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LootPoolJson {
    #[serde(default = "default_rolls")]
    rolls: Value,
    #[serde(default)]
    bonus_rolls: f32,
    #[serde(default)]
    entries: Vec<LootEntryJson>,
    #[serde(default)]
    conditions: Vec<LootConditionJson>,
}

fn default_rolls() -> Value {
    Value::Number(serde_json::Number::from(1))
}

#[derive(Deserialize, Debug)]
struct LootEntryJson {
    #[serde(rename = "type")]
    entry_type: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default = "default_weight")]
    weight: i32,
    #[serde(default)]
    quality: i32,
    #[serde(default)]
    expand: bool,
    #[serde(default)]
    conditions: Vec<LootConditionJson>,
    #[serde(default)]
    functions: Vec<LootFunctionJson>,
    #[serde(default)]
    children: Vec<LootEntryJson>,
}

fn default_weight() -> i32 {
    1
}

#[derive(Deserialize, Debug)]
struct LootConditionJson {
    condition: String,
    // Other fields vary by condition type
}

#[derive(Deserialize, Debug)]
struct LootFunctionJson {
    function: String,
    #[serde(default)]
    count: Option<Value>,
    #[serde(default)]
    add: bool,
    // Other fields vary by function type
}

/// Generate a NumberProvider token stream from a JSON value.
fn generate_number_provider(value: &Value) -> TokenStream {
    match value {
        Value::Number(n) => {
            let v = n.as_f64().unwrap_or(1.0) as f32;
            quote! { NumberProvider::Constant(#v) }
        }
        Value::Object(obj) => {
            let type_str = obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("minecraft:constant");

            match type_str {
                "minecraft:uniform" => {
                    let min = obj.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    let max = obj.get("max").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
                    quote! { NumberProvider::Uniform { min: #min, max: #max } }
                }
                "minecraft:binomial" => {
                    let n = obj.get("n").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
                    let p = obj.get("p").and_then(|v| v.as_f64()).unwrap_or(0.5) as f32;
                    quote! { NumberProvider::Binomial { n: #n, p: #p } }
                }
                "minecraft:constant" => {
                    let v = obj.get("value").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
                    quote! { NumberProvider::Constant(#v) }
                }
                _ => quote! { NumberProvider::Constant(1.0) },
            }
        }
        _ => quote! { NumberProvider::Constant(1.0) },
    }
}

/// Generate a LootCondition token stream.
fn generate_condition(condition: &LootConditionJson) -> TokenStream {
    match condition.condition.as_str() {
        "minecraft:survives_explosion" => {
            quote! { LootCondition::SurvivesExplosion }
        }
        _ => {
            quote! { LootCondition::Unknown }
        }
    }
}

/// Generate a LootFunction token stream.
fn generate_function(function: &LootFunctionJson) -> TokenStream {
    match function.function.as_str() {
        "minecraft:set_count" => {
            let count = function
                .count
                .as_ref()
                .map(generate_number_provider)
                .unwrap_or_else(|| quote! { NumberProvider::Constant(1.0) });
            let add = function.add;
            quote! { LootFunction::SetCount { count: #count, add: #add } }
        }
        _ => {
            quote! { LootFunction::Unknown }
        }
    }
}

/// Generate a LootEntry token stream recursively.
fn generate_entry(entry: &LootEntryJson) -> TokenStream {
    let conditions: Vec<TokenStream> = entry.conditions.iter().map(generate_condition).collect();
    let functions: Vec<TokenStream> = entry.functions.iter().map(generate_function).collect();

    match entry.entry_type.as_str() {
        "minecraft:item" => {
            let name = entry.name.as_deref().unwrap_or("minecraft:air");
            let name = name.strip_prefix("minecraft:").unwrap_or(name);
            let weight = entry.weight;
            let quality = entry.quality;
            quote! {
                LootEntry::Item {
                    name: Identifier::vanilla_static(#name),
                    weight: #weight,
                    quality: #quality,
                    conditions: &[#(#conditions),*],
                    functions: &[#(#functions),*],
                }
            }
        }
        "minecraft:loot_table" => {
            let name = entry.name.as_deref().unwrap_or("minecraft:empty");
            let name = name.strip_prefix("minecraft:").unwrap_or(name);
            let weight = entry.weight;
            let quality = entry.quality;
            quote! {
                LootEntry::LootTableRef {
                    name: Identifier::vanilla_static(#name),
                    weight: #weight,
                    quality: #quality,
                    conditions: &[#(#conditions),*],
                    functions: &[#(#functions),*],
                }
            }
        }
        "minecraft:tag" => {
            let name = entry.name.as_deref().unwrap_or("minecraft:empty");
            let name = name.strip_prefix("minecraft:").unwrap_or(name);
            let expand = entry.expand;
            let weight = entry.weight;
            let quality = entry.quality;
            quote! {
                LootEntry::Tag {
                    name: Identifier::vanilla_static(#name),
                    expand: #expand,
                    weight: #weight,
                    quality: #quality,
                    conditions: &[#(#conditions),*],
                    functions: &[#(#functions),*],
                }
            }
        }
        "minecraft:alternatives" => {
            let children: Vec<TokenStream> = entry.children.iter().map(generate_entry).collect();
            quote! {
                LootEntry::Alternatives {
                    children: &[#(#children),*],
                    conditions: &[#(#conditions),*],
                }
            }
        }
        "minecraft:group" => {
            let children: Vec<TokenStream> = entry.children.iter().map(generate_entry).collect();
            quote! {
                LootEntry::Group {
                    children: &[#(#children),*],
                    conditions: &[#(#conditions),*],
                }
            }
        }
        "minecraft:sequence" => {
            let children: Vec<TokenStream> = entry.children.iter().map(generate_entry).collect();
            quote! {
                LootEntry::Sequence {
                    children: &[#(#children),*],
                    conditions: &[#(#conditions),*],
                }
            }
        }
        "minecraft:empty" => {
            let weight = entry.weight;
            quote! {
                LootEntry::Empty {
                    weight: #weight,
                    conditions: &[#(#conditions),*],
                }
            }
        }
        "minecraft:dynamic" => {
            let name = entry.name.as_deref().unwrap_or("contents");
            let name = name.strip_prefix("minecraft:").unwrap_or(name);
            quote! {
                LootEntry::Dynamic {
                    name: Identifier::vanilla_static(#name),
                    conditions: &[#(#conditions),*],
                }
            }
        }
        _ => {
            // Fallback to empty
            quote! {
                LootEntry::Empty {
                    weight: 1,
                    conditions: &[],
                }
            }
        }
    }
}

/// Generate a LootPool token stream.
fn generate_pool(pool: &LootPoolJson) -> TokenStream {
    let rolls = generate_number_provider(&pool.rolls);
    let bonus_rolls = pool.bonus_rolls;
    let entries: Vec<TokenStream> = pool.entries.iter().map(generate_entry).collect();
    let conditions: Vec<TokenStream> = pool.conditions.iter().map(generate_condition).collect();

    quote! {
        LootPool {
            rolls: #rolls,
            bonus_rolls: #bonus_rolls,
            entries: &[#(#entries),*],
            conditions: &[#(#conditions),*],
        }
    }
}

struct LootTableData {
    /// Full key path like "blocks/acacia_button"
    key: String,
    /// Rust identifier like "BLOCKS_ACACIA_BUTTON"
    const_ident: Ident,
    /// The loot type
    loot_type: String,
    /// Generated pools
    pools: Vec<TokenStream>,
    /// Random sequence identifier
    random_sequence: Option<String>,
}

pub(crate) fn build() -> TokenStream {
    println!(
        "cargo:rerun-if-changed=build_assets/builtin_datapacks/minecraft/data/minecraft/loot_table/"
    );

    let loot_table_dir = "build_assets/builtin_datapacks/minecraft/data/minecraft/loot_table";
    let mut tables: Vec<LootTableData> = Vec::new();

    // Recursively read all loot table JSON files
    fn read_loot_tables(dir: &Path, base_dir: &Path, tables: &mut Vec<LootTableData>) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                read_loot_tables(&path, base_dir, tables);
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let relative_path = path
                    .strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .with_extension("");
                let key = relative_path
                    .to_str()
                    .unwrap_or("unknown")
                    .replace('\\', "/");

                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                let loot_table: LootTableJson = match serde_json::from_str(&content) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Failed to parse loot table {}: {}", key, e);
                        continue;
                    }
                };

                // Generate const identifier from the key
                let const_name = key.replace('/', "_").to_shouty_snake_case();
                let const_ident = Ident::new(&const_name, Span::call_site());

                let pools: Vec<TokenStream> = loot_table.pools.iter().map(generate_pool).collect();

                let random_sequence = loot_table
                    .random_sequence
                    .as_ref()
                    .map(|s| s.strip_prefix("minecraft:").unwrap_or(s).to_string());

                tables.push(LootTableData {
                    key,
                    const_ident,
                    loot_type: loot_table
                        .loot_type
                        .unwrap_or_else(|| "minecraft:empty".to_string()),
                    pools,
                    random_sequence,
                });
            }
        }
    }

    read_loot_tables(
        Path::new(loot_table_dir),
        Path::new(loot_table_dir),
        &mut tables,
    );

    // Sort by key for consistent generation
    tables.sort_by(|a, b| a.key.cmp(&b.key));

    let mut stream = TokenStream::new();

    // Imports
    stream.extend(quote! {
        use crate::loot_table::{
            LootCondition, LootEntry, LootFunction, LootPool, LootTable,
            LootTableRef, LootTableRegistry, LootType, NumberProvider,
        };
        use steel_utils::Identifier;
    });

    // Generate static constants for each loot table
    for table in &tables {
        let const_ident = &table.const_ident;
        let key = &table.key;
        let loot_type = &table.loot_type;
        let pools = &table.pools;

        let random_sequence = match &table.random_sequence {
            Some(seq) => quote! { Some(Identifier::vanilla_static(#seq)) },
            None => quote! { None },
        };

        stream.extend(quote! {
            pub const #const_ident: &LootTable = &LootTable {
                key: Identifier::vanilla_static(#key),
                loot_type: LootType::from_str(#loot_type),
                pools: &[#(#pools),*],
                random_sequence: #random_sequence,
            };
        });
    }

    // Generate registration function
    let register_calls: Vec<TokenStream> = tables
        .iter()
        .map(|t| {
            let const_ident = &t.const_ident;
            quote! { registry.register(#const_ident); }
        })
        .collect();

    stream.extend(quote! {
        pub fn register_loot_tables(registry: &mut LootTableRegistry) {
            #(#register_calls)*
        }
    });

    // Generate a struct with categorized access for convenience
    // Group tables by their top-level directory
    let mut categories: std::collections::BTreeMap<String, Vec<(&LootTableData, Ident)>> =
        std::collections::BTreeMap::new();

    for table in &tables {
        let category = table.key.split('/').next().unwrap_or("other").to_string();
        let field_name = table
            .key
            .split('/')
            .skip(1)
            .collect::<Vec<_>>()
            .join("_")
            .to_snake_case();
        let field_name = if field_name.is_empty() {
            table.key.to_snake_case()
        } else {
            field_name
        };
        let field_ident = Ident::new(&field_name, Span::call_site());
        categories
            .entry(category)
            .or_default()
            .push((table, field_ident));
    }

    // Generate category structs
    for (category, items) in &categories {
        let struct_name = Ident::new(
            &format!(
                "{}LootTables",
                category
                    .to_snake_case()
                    .replace('_', " ")
                    .split_whitespace()
                    .map(|s| {
                        let mut c = s.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<String>()
            ),
            Span::call_site(),
        );

        let fields: Vec<TokenStream> = items
            .iter()
            .map(|(_, field_ident)| {
                quote! { pub #field_ident: LootTableRef, }
            })
            .collect();

        let inits: Vec<TokenStream> = items
            .iter()
            .map(|(table, field_ident)| {
                let const_ident = &table.const_ident;
                quote! { #field_ident: #const_ident, }
            })
            .collect();

        stream.extend(quote! {
            pub struct #struct_name {
                #(#fields)*
            }

            impl #struct_name {
                pub const fn new() -> Self {
                    Self {
                        #(#inits)*
                    }
                }
            }
        });
    }

    // Generate the main LOOT_TABLES struct
    let category_fields: Vec<TokenStream> = categories
        .keys()
        .map(|category| {
            let field_ident = Ident::new(&category.to_snake_case(), Span::call_site());
            let struct_name = Ident::new(
                &format!(
                    "{}LootTables",
                    category
                        .to_snake_case()
                        .replace('_', " ")
                        .split_whitespace()
                        .map(|s| {
                            let mut c = s.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        })
                        .collect::<String>()
                ),
                Span::call_site(),
            );
            quote! { pub #field_ident: #struct_name, }
        })
        .collect();

    let category_inits: Vec<TokenStream> = categories
        .keys()
        .map(|category| {
            let field_ident = Ident::new(&category.to_snake_case(), Span::call_site());
            let struct_name = Ident::new(
                &format!(
                    "{}LootTables",
                    category
                        .to_snake_case()
                        .replace('_', " ")
                        .split_whitespace()
                        .map(|s| {
                            let mut c = s.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        })
                        .collect::<String>()
                ),
                Span::call_site(),
            );
            quote! { #field_ident: #struct_name::new(), }
        })
        .collect();

    stream.extend(quote! {
        pub struct LootTables {
            #(#category_fields)*
        }

        impl LootTables {
            pub const fn new() -> Self {
                Self {
                    #(#category_inits)*
                }
            }
        }

        pub static LOOT_TABLES: LootTables = LootTables::new();
    });

    stream
}
