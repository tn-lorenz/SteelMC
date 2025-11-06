use std::marker::PhantomData;

use crate::text::TextComponent;

/// Marker types for argument counts
pub struct Args0;
pub struct Args1;
pub struct Args2;
pub struct Args3;
pub struct Args4;
pub struct Args5;
pub struct Args6;
pub struct Args7;
pub struct Args8;
pub struct Args9;

/// A translation with compile-time argument count checking
#[derive(Clone, Copy)]
pub struct Translation<T> {
    pub key: &'static str,
    pub format: &'static str,
    _marker: PhantomData<T>,
}

impl<T> Translation<T> {
    pub const fn new(key: &'static str, format: &'static str) -> Self {
        Self {
            key,
            format,
            _marker: PhantomData,
        }
    }
}

impl Translation<Args0> {
    pub fn message(self) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([]),
        }
    }
}

impl Translation<Args1> {
    pub fn message(self, arg: impl Into<TextComponent>) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([arg.into()]),
        }
    }
}

impl Translation<Args2> {
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([arg1.into(), arg2.into()]),
        }
    }
}

impl Translation<Args3> {
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([arg1.into(), arg2.into(), arg3.into()]),
        }
    }
}

impl Translation<Args4> {
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([arg1.into(), arg2.into(), arg3.into(), arg4.into()]),
        }
    }
}

impl Translation<Args5> {
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
        arg5: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([
                arg1.into(),
                arg2.into(),
                arg3.into(),
                arg4.into(),
                arg5.into(),
            ]),
        }
    }
}

impl Translation<Args6> {
    #[allow(clippy::too_many_arguments)]
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
        arg5: impl Into<TextComponent>,
        arg6: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([
                arg1.into(),
                arg2.into(),
                arg3.into(),
                arg4.into(),
                arg5.into(),
                arg6.into(),
            ]),
        }
    }
}

impl Translation<Args7> {
    #[allow(clippy::too_many_arguments)]
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
        arg5: impl Into<TextComponent>,
        arg6: impl Into<TextComponent>,
        arg7: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([
                arg1.into(),
                arg2.into(),
                arg3.into(),
                arg4.into(),
                arg5.into(),
                arg6.into(),
                arg7.into(),
            ]),
        }
    }
}

impl Translation<Args8> {
    #[allow(clippy::too_many_arguments)]
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
        arg5: impl Into<TextComponent>,
        arg6: impl Into<TextComponent>,
        arg7: impl Into<TextComponent>,
        arg8: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([
                arg1.into(),
                arg2.into(),
                arg3.into(),
                arg4.into(),
                arg5.into(),
                arg6.into(),
                arg7.into(),
                arg8.into(),
            ]),
        }
    }
}

impl Translation<Args9> {
    #[allow(clippy::too_many_arguments)]
    pub fn message(
        self,
        arg1: impl Into<TextComponent>,
        arg2: impl Into<TextComponent>,
        arg3: impl Into<TextComponent>,
        arg4: impl Into<TextComponent>,
        arg5: impl Into<TextComponent>,
        arg6: impl Into<TextComponent>,
        arg7: impl Into<TextComponent>,
        arg8: impl Into<TextComponent>,
        arg9: impl Into<TextComponent>,
    ) -> TranslatedMessage {
        TranslatedMessage {
            key: self.key,
            format: self.format,
            args: Box::new([
                arg1.into(),
                arg2.into(),
                arg3.into(),
                arg4.into(),
                arg5.into(),
                arg6.into(),
                arg7.into(),
                arg8.into(),
                arg9.into(),
            ]),
        }
    }
}

/// A constructed message
pub struct TranslatedMessage {
    key: &'static str,
    format: &'static str,
    args: Box<[TextComponent]>,
}

impl TranslatedMessage {
    /// Get the translation key (for sending to client)
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// Get the arguments as a slice
    pub fn args(&self) -> &[TextComponent] {
        &self.args
    }

    /// Get key and arguments for client packet
    pub fn client_data(&self) -> (&str, &[TextComponent]) {
        (self.key, &self.args)
    }

    /// Format the message on the server-side
    pub fn format(&self) -> String {
        let mut result = self.format.to_string();

        // Handle positional arguments
        for (i, arg) in self.args.iter().enumerate() {
            result = result.replace(&format!("%{}$s", i + 1), &arg.to_string());
        }

        // Handle sequential %s
        for arg in self.args.iter() {
            result = result.replacen("%s", &arg.to_string(), 1);
        }

        result
    }
}
