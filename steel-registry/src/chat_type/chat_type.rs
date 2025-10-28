use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a chat type definition from the data packs.
#[derive(Debug)]
pub struct ChatType {
    pub key: ResourceLocation,
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}

/// Defines the styling and translation for a part of a chat message.
#[derive(Debug)]
pub struct ChatTypeDecoration {
    pub translation_key: &'static str,
    pub parameters: &'static [&'static str],
    pub style: Option<ChatStyle>,
}

/// Defines optional text styling, like color and formatting.
#[derive(Debug)]
pub struct ChatStyle {
    pub color: Option<&'static str>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
}

pub type ChatTypeRef = &'static ChatType;

pub struct ChatTypeRegistry {
    chat_types: HashMap<ResourceLocation, ChatTypeRef>,
    allows_registering: bool,
}

impl ChatTypeRegistry {
    pub fn new() -> Self {
        Self {
            chat_types: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, chat_type: ChatTypeRef) {
        if !self.allows_registering {
            panic!("Cannot register chat types after the registry has been frozen");
        }

        self.chat_types.insert(chat_type.key.clone(), chat_type);
    }
}

impl RegistryExt for ChatTypeRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
