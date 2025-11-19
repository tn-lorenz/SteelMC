#![allow(missing_docs)]

use std::{pin::Pin, sync::Arc};

use futures::Future;

use crate::chunk::{
    chunk_access::{ChunkAccess, ChunkStatus},
    chunk_generation_task::StaticCache2D,
    chunk_holder::ChunkHolder,
    chunk_pyramid::ChunkStep,
    proto_chunk::ProtoChunk,
    section::{ChunkSection, Sections},
    world_gen_context::WorldGenContext,
};

pub struct ChunkStatusTasks;

impl ChunkStatusTasks {
    pub fn empty(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move {
            // TODO: Check if chunk exists on disk and load it.
            // For now, create a new empty chunk.
            let sections = (0..24) // Standard height?
                .map(|_| ChunkSection::new_empty())
                .collect::<Vec<_>>()
                .into_boxed_slice();

            // TODO: Use upgrade_to_full if the loaded chunk is full.
            let proto_chunk = ProtoChunk {
                sections: Sections { sections },
                pos: holder.get_pos(),
            };

            //log::info!("Inserted proto chunk for {:?}", holder.get_pos());

            holder.insert_chunk(ChunkAccess::Proto(proto_chunk), ChunkStatus::Empty);
            Ok(())
        })
    }

    pub fn generate_structure_starts(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_structure_references(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn load_structure_starts(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_biomes(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_noise(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_surface(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_carvers(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_features(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn initialize_light(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn light(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn generate_spawn(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        _holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        Box::pin(async move { Ok(()) })
    }

    pub fn full(
        _context: Arc<WorldGenContext>,
        _step: &Arc<ChunkStep>,
        _cache: &Arc<StaticCache2D<Arc<ChunkHolder>>>,
        holder: Arc<ChunkHolder>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + Sync,
        >,
    > {
        //panic!("Full task");
        Box::pin(async move {
            holder.upgrade_to_full();
            Ok(())
        })
    }
}
