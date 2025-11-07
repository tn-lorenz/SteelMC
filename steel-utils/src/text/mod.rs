use serde::{Deserialize, Serialize};
use simdnbt::{
    ToNbtTag,
    owned::{NbtCompound, NbtList, NbtTag},
};
use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use crate::text::{
    color::Color, interactivity::Interactivity, style::Style, translation::TranslatedMessage,
};

pub mod click;
pub mod color;
pub mod hover;
pub mod interactivity;
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
    #[serde(flatten)]
    pub style: Style,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Extra text components
    pub extra: Vec<TextComponent>,
    /// Interactivity of the text. Click event, hover event, etc.
    #[serde(flatten)]
    pub interactivity: Interactivity,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TextContent {
    /// Raw text
    Text { text: Cow<'static, str> },
    /// Translated text
    Translate(TranslatedMessage),
    /// A keybind identifier
    /// https://minecraft.wiki/w/Controls#Configurable_controls
    Keybind { keybind: Cow<'static, str> },
}

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self::new().text(value)
    }
}

impl From<&'static str> for TextComponent {
    fn from(value: &'static str) -> Self {
        Self::new().text(value)
    }
}

impl Default for TextComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl TextComponent {
    pub const fn new() -> Self {
        Self {
            content: TextContent::Text {
                text: Cow::Borrowed(""),
            },
            style: Style::new(),
            extra: Vec::new(),
            interactivity: Interactivity::new(),
        }
    }

    pub fn translate(mut self, translation: TranslatedMessage) -> Self {
        self.content = TextContent::Translate(translation);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.content = TextContent::Text { text: text.into() };
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style = self.style.color(color);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn extra(mut self, extra: impl Into<TextComponent>) -> Self {
        self.extra.push(extra.into());
        self
    }

    pub fn interactivity(mut self, interactivity: Interactivity) -> Self {
        self.interactivity = interactivity;
        self
    }

    pub const fn const_text(text: &'static str) -> Self {
        Self {
            content: TextContent::Text {
                text: Cow::Borrowed(text),
            },
            style: Style::new(),
            extra: Vec::new(),
            interactivity: Interactivity::new(),
        }
    }

    pub const fn text_with_color(text: Cow<'static, str>, color: Color) -> Self {
        Self {
            content: TextContent::Text { text },
            style: Style::new().color(color),
            extra: Vec::new(),
            interactivity: Interactivity::new(),
        }
    }

    pub const fn const_translate(key: &'static str) -> Self {
        Self {
            content: TextContent::Translate(TranslatedMessage::new(key, None)),
            style: Style::new(),
            extra: Vec::new(),
            interactivity: Interactivity::new(),
        }
    }

    pub const fn const_translate_with_color(key: &'static str, color: Color) -> Self {
        Self {
            content: TextContent::Translate(TranslatedMessage::new(key, None)),
            style: Style::new().color(color),
            extra: Vec::new(),
            interactivity: Interactivity::new(),
        }
    }

    pub fn into_nbt_compound(self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        match self.content {
            TextContent::Text { text } => compound.insert("text", text.to_string()),
            TextContent::Translate(message) => {
                compound.insert("translate", message.key.to_string());
                if let Some(fallback) = message.fallback {
                    compound.insert("fallback", fallback.to_string());
                }
                if let Some(args) = message.args {
                    compound.insert(
                        "with",
                        NbtTag::List(NbtList::Compound(
                            args.into_iter()
                                .map(|arg| arg.into_nbt_compound())
                                .collect(),
                        )),
                    );
                }
            }
            TextContent::Keybind { keybind } => compound.insert("keybind", keybind.to_string()),
        }

        compound.extend(self.style.into_nbt_compound());

        if !self.extra.is_empty() {
            compound.insert(
                "extra",
                NbtList::Compound(
                    self.extra
                        .into_iter()
                        .map(|extra| extra.into_nbt_compound())
                        .collect(),
                ),
            );
        }

        log::info!("compound: {:?}", compound);
        compound
    }
}

impl ToNbtTag for TextComponent {
    fn to_nbt_tag(self) -> NbtTag {
        NbtTag::Compound(self.into_nbt_compound())
    }
}

impl Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            TextContent::Text { text } => write!(f, "{}", text),
            TextContent::Translate(message) => write!(f, "{}", message.format()),
            _ => unimplemented!(),
        }
    }
}
