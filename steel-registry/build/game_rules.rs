use std::fs;

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct GameRulesFile {
    game_rules: Vec<GameRuleEntry>,
}

#[derive(Deserialize)]
struct GameRuleEntry {
    name: String,
    category: String,
    #[serde(rename = "type")]
    value_type: String,
    default: serde_json::Value,
}

pub fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=build_assets/game_rules.json");

    let json_content =
        fs::read_to_string("build_assets/game_rules.json").expect("Failed to read game_rules.json");
    let game_rules_file: GameRulesFile =
        serde_json::from_str(&json_content).expect("Failed to parse game_rules.json");

    let mut constants = TokenStream::new();
    let mut registrations = TokenStream::new();

    for rule in &game_rules_file.game_rules {
        let const_name = Ident::new(&rule.name.to_shouty_snake_case(), Span::call_site());
        let rule_name = Literal::string(&rule.name);
        let category = Ident::new(&rule.category, Span::call_site());

        match rule.value_type.as_str() {
            "bool" => {
                let default_value = rule.default.as_bool().unwrap_or(false);
                constants.extend(quote! {
                    pub const #const_name: &GameRule<bool> = &GameRule {
                        key: Identifier::vanilla_static(#rule_name),
                        category: GameRuleCategory::#category,
                        default_value: #default_value,
                    };
                });
            }
            "int" => {
                let default_value = rule.default.as_i64().unwrap_or(0) as i32;
                constants.extend(quote! {
                    pub const #const_name: &GameRule<i32> = &GameRule {
                        key: Identifier::vanilla_static(#rule_name),
                        category: GameRuleCategory::#category,
                        default_value: #default_value,
                    };
                });
            }
            _ => panic!("Unknown game rule type: {}", rule.value_type),
        }

        registrations.extend(quote! {
            registry.register(#const_name);
        });
    }

    quote! {
        use crate::game_rules::{GameRule, GameRuleRegistry, GameRuleCategory};
        use steel_utils::Identifier;

        #constants

        pub fn register_game_rules(registry: &mut GameRuleRegistry) {
            #registrations
        }
    }
}
