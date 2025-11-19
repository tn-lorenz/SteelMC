use std::sync::Arc;

use crate::chunk::chunk_generator::ChunkGenerator;

pub struct WorldGenContext {
    pub generator: Arc<dyn ChunkGenerator>,
    // Add other fields as needed:
    // pub level: ServerLevel,
    // pub structure_manager: StructureTemplateManager,
    // pub light_engine: ThreadedLevelLightEngine,
    // pub main_thread_executor: Executor,
    // pub unsaved_listener: UnsavedListener,
}
