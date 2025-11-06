use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::text::TextComponent;

/// A translation with compile-time argument count checking
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Translation<const ARGS: usize> {
    pub key: &'static str,
    pub format: &'static str,
}

impl<const ARGS: usize> Translation<ARGS> {
    pub const fn new(key: &'static str, format: &'static str) -> Self {
        Self { key, format }
    }
}

impl Translation<0> {
    pub fn msg(&self) -> TranslatedMessage {
        TranslatedMessage::new(self.key, None)
    }
}

impl<const ARGS: usize> Translation<ARGS> {
    pub fn message(self, args: [impl Into<TextComponent>; ARGS]) -> TranslatedMessage {
        TranslatedMessage::new(self.key, Some(Box::new(args.map(|a| a.into()))))
    }
}

/// A constructed message
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslatedMessage {
    pub key: Cow<'static, str>,
    pub args: Option<Box<[TextComponent]>>,
}

pub const EMPTY_ARGS: &[TextComponent] = &[];

impl TranslatedMessage {
    pub const fn new(key: &'static str, args: Option<Box<[TextComponent]>>) -> Self {
        Self {
            key: Cow::Borrowed(key),
            args,
        }
    }

    /// Get the translation key (for sending to client)
    pub fn key(&self) -> Cow<'static, str> {
        self.key.clone()
    }

    /// Get the arguments as a slice
    pub fn args(&self) -> &[TextComponent] {
        match &self.args {
            Some(args) => args,
            None => EMPTY_ARGS,
        }
    }

    /// Get key and arguments for client packet
    pub fn client_data(&self) -> (Cow<'static, str>, &[TextComponent]) {
        (self.key.clone(), self.args())
    }

    /// Format the message on the server-side
    pub fn format(&self) -> String {
        let mut result = crate::translations::TRANSLATIONS
            .get(self.key.as_ref())
            .unwrap()
            .to_string();

        // Handle positional arguments
        for (i, arg) in self.args().iter().enumerate() {
            result = result.replace(&format!("%{}$s", i + 1), &arg.to_string());
        }

        // Handle sequential %s
        for arg in self.args().iter() {
            result = result.replacen("%s", &arg.to_string(), 1);
        }

        result
    }
}
