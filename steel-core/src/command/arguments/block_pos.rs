//! A block position argument.

use steel_protocol::packets::game::{ArgumentType, SuggestionType};
use steel_utils::BlockPos;

use crate::command::arguments::CommandArgument;
use crate::command::context::CommandContext;

/// A block position argument.
pub struct BlockPosArgument;

impl CommandArgument for BlockPosArgument {
    type Output = BlockPos;

    fn parse<'a>(
        &self,
        arg: &'a [&'a str],
        context: &mut CommandContext,
    ) -> Option<(&'a [&'a str], Self::Output)> {
        let x = parse_coordinate(arg.first()?, context.position.x)?;
        let y = parse_coordinate(arg.get(1)?, context.position.y)?;
        let z = parse_coordinate(arg.get(2)?, context.position.z)?;

        Some((&arg[3..], BlockPos::containing(x, y, z)))
    }

    fn usage(&self) -> (ArgumentType, Option<SuggestionType>) {
        (ArgumentType::BlockPos, None)
    }
}

fn parse_coordinate(value: &str, origin: f64) -> Option<f64> {
    // TODO: support vanilla local coordinates (`^`) once command coordinate rotation is shared.
    if let Some(offset) = value.strip_prefix('~') {
        if offset.is_empty() {
            Some(origin)
        } else {
            Some(origin + offset.parse::<f64>().ok()?)
        }
    } else {
        Some(f64::from(value.parse::<i32>().ok()?))
    }
}
