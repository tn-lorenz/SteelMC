//! An integer argument.
use steel_protocol::packets::game::{ArgumentType, SuggestionType};

use crate::command::arguments::CommandArgument;
use crate::command::context::CommandContext;

/// An integer argument that parses a 32-bit signed integer.
pub struct IntegerArgument;

impl CommandArgument for IntegerArgument {
    type Output = i32;

    fn parse<'a>(
        &self,
        arg: &'a [&'a str],
        _context: &mut CommandContext,
    ) -> Option<(&'a [&'a str], Self::Output)> {
        let s = arg.first()?;
        let value = s.parse().ok()?;
        Some((&arg[1..], value))
    }

    fn usage(&self) -> (ArgumentType, Option<SuggestionType>) {
        (
            ArgumentType::Integer {
                min: None,
                max: None,
            },
            None,
        )
    }
}
