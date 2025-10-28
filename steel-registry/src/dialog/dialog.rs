use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents the different types of dialogs defined in data packs.
#[derive(Debug)]
pub enum Dialog {
    DialogList(DialogList),
    ServerLinks(ServerLinks),
}

/// A dialog that displays a list of other dialogs.
#[derive(Debug)]
pub struct DialogList {
    pub key: ResourceLocation,
    pub button_width: i32,
    pub columns: i32,
    pub dialogs: &'static str,
    pub exit_action: ExitAction,
    pub external_title: TextComponent,
    pub title: TextComponent,
}

/// A dialog that displays server links.
#[derive(Debug)]
pub struct ServerLinks {
    pub key: ResourceLocation,
    pub button_width: i32,
    pub columns: i32,
    pub exit_action: ExitAction,
    pub external_title: TextComponent,
    pub title: TextComponent,
}

/// Represents an exit action with a label and width.
#[derive(Debug)]
pub struct ExitAction {
    pub label: TextComponent,
    pub width: i32,
}

/// Represents a text component with translation key.
#[derive(Debug)]
pub struct TextComponent {
    pub translate: &'static str,
}

pub type DialogRef = &'static Dialog;

pub struct DialogRegistry {
    dialogs: HashMap<ResourceLocation, DialogRef>,
    allows_registering: bool,
}

impl DialogRegistry {
    pub fn new() -> Self {
        Self {
            dialogs: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, key: ResourceLocation, dialog: DialogRef) {
        if !self.allows_registering {
            panic!("Cannot register dialogs after the registry has been frozen");
        }

        self.dialogs.insert(key, dialog);
    }
}

impl RegistryExt for DialogRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
