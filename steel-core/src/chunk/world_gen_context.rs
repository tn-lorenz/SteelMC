//! This module contains the `WorldGenContext` struct, which is used to provide context for chunk generation.
use std::sync::Arc;

use crate::chunk::chunk_generator::ChunkGenerator;

/// Context for world generation.
pub struct WorldGenContext {
    /// The chunk generator to use.
    pub generator: Arc<dyn ChunkGenerator>,
    // Add other fields as needed:
    // pub level: ServerLevel,
    // pub structure_manager: StructureTemplateManager,
    // pub light_engine: ThreadedLevelLightEngine,
    // pub main_thread_executor: Executor,
    // pub unsaved_listener: UnsavedListener,
}
