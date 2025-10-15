use std::sync::Arc;

use steel_utils::{SteelRwLock, math::vector2::Vector2};
use steel_world::{
    BlockStateId, ChunkData, Level,
    section::{BlockPalette, ChunkSections, SubChunk},
};

const TEMP_OVERWORLD_HEIGHT: usize = 384;

#[tokio::main]
async fn main() {
    let level = Level {
        chunks: papaya::HashMap::new(),
    };

    let section_count = TEMP_OVERWORLD_HEIGHT / BlockPalette::SIZE;
    let sections: Box<[SubChunk]> = (0..section_count)
        .map(|_| SubChunk {
            block_states: BlockPalette::Homogeneous(0),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    level.chunks.pin().insert(
        Vector2::new(0, 0),
        Arc::new(SteelRwLock::new(ChunkData {
            sections: ChunkSections::new(sections, -64),
        })),
    );

    {
        let pin = level.chunks.pin();
        let chunk = pin.get(&Vector2::new(0, 0)).unwrap().clone();

        chunk
            .write()
            .await
            .sections
            .set_relative_block(0, 0, 0, BlockStateId(1));
    }

    println!("{:#?}", level.chunks.pin().get(&Vector2::new(0, 0)));
}
