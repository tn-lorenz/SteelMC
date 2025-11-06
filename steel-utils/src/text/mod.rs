use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use crate::text::{style::Style, translation::TranslatedMessage};

pub mod click;
pub mod color;
pub mod hover;
pub mod locale;
pub mod style;
pub mod translation;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent {
    /// The actual text
    #[serde(flatten)]
    pub content: TextContent,
    /// Style of the text. Bold, Italic, underline, Color...
    /// Also has `ClickEvent
    #[serde(flatten)]
    pub style: Style,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Extra text components
    pub extra: Vec<TextComponent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TextContent {
    /// Raw text
    Text { text: Cow<'static, str> },
    /// Translated text
    Translate {
        translate: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<TextComponent>,
    },
    /// Displays the name of one or more entities found by a selector.
    EntityNames {
        selector: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        separator: Option<Cow<'static, str>>,
    },
    /// A keybind identifier
    /// https://minecraft.wiki/w/Controls#Configurable_controls
    Keybind { keybind: Cow<'static, str> },
}

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self::text(value)
    }
}

impl From<&'static str> for TextComponent {
    fn from(value: &'static str) -> Self {
        Self::text(value)
    }
}

impl TextComponent {
    pub fn text<P: Into<Cow<'static, str>>>(plain: P) -> Self {
        Self {
            content: TextContent::Text { text: plain.into() },
            style: Style::default(),
            extra: vec![],
        }
    }

    pub fn translate(message: TranslatedMessage) -> Self {
        Self {
            content: TextContent::Translate {
                translate: Cow::Borrowed(message.key()),
                with: message.args().into(),
            },
            style: Style::default(),
            extra: vec![],
        }
    }

    /// Create a simple translated text component in a const context
    /// This is useful for static/const definitions where the full `translate` method cannot be used
    pub const fn const_translate(key: &'static str) -> Self {
        Self {
            content: TextContent::Translate {
                translate: Cow::Borrowed(key),
                with: Vec::new(),
            },
            style: Style {
                color: None,
                bold: None,
                italic: None,
                underlined: None,
                strikethrough: None,
                obfuscated: None,
                insertion: None,
                click_event: None,
                hover_event: None,
                font: None,
                shadow_color: None,
            },
            extra: Vec::new(),
        }
    }

    /// Create a translated text component with a color in a const context
    pub const fn const_translate_with_color(key: &'static str, color: color::Color) -> Self {
        Self {
            content: TextContent::Translate {
                translate: Cow::Borrowed(key),
                with: Vec::new(),
            },
            style: Style {
                color: Some(color),
                bold: None,
                italic: None,
                underlined: None,
                strikethrough: None,
                obfuscated: None,
                insertion: None,
                click_event: None,
                hover_event: None,
                font: None,
                shadow_color: None,
            },
            extra: Vec::new(),
        }
    }
}

impl Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            TextContent::Text { text } => write!(f, "{}", text),
            _ => unimplemented!(),
        }
    }
}
