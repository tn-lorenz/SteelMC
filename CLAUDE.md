# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Steel is a lightweight Rust implementation of a Minecraft Java Edition server (currently targeting version 1.21.11), partially based on [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin). The project is in early development and emphasizes clean code, performance, extensibility, and ease of use.

## Build and Development Commands

**Basic commands:**
```bash
# Build the project
cargo build --release

# Run the server
cargo run --release

# Run tests
cargo test --verbose

# Format code
cargo fmt

# Run linter
cargo clippy --release --all-targets --all-features

# Check compilation without building
cargo check
```

**Requirements:**
- Rust nightly toolchain (the project uses edition 2024 and nightly features)
- Install with: `rustup toolchain install nightly && rustup default nightly`

## Workspace Architecture

Steel uses a Cargo workspace with 7 specialized crates:

1. **steel** - Main binary, server orchestration, TCP listening, connection handling
2. **steel-core** - Game logic (PLAY state), chunk system, player management, world, inventory, commands
3. **steel-protocol** - Network protocol layer, packet serialization/deserialization, packet definitions
4. **steel-registry** - Game data registries (blocks, items, biomes, etc.), mostly generated code
5. **steel-utils** - Shared utilities (codecs, math, text components, types, locks)
6. **steel-macros** - Procedural macros for packet serialization
7. **steel-crypto** - Cryptography layer (RSA keys, Mojang authentication, signature validation)

## High-Level Architecture

**Three-Layer Model:**
```
Network Layer (steel)
  ↓ TCP connections, packet encoding/decoding
Protocol Layer (steel-protocol)
  ↓ Deserialized packets
Game Logic Layer (steel-core)
  ↓ Server, World, Player, Chunk management
```

**Dual Runtime Pattern:**
- **Main Runtime**: Server tick loop (20 TPS), player ticking, world ticking
- **Chunk Runtime**: Separate async runtime for chunk generation/loading to prevent blocking the main tick loop
- Communication between runtimes uses `block_on` for spawn_blocking calls

**Key Flow:**
1. TCP connection → `JavaTcpClient` created
2. Handshake state → determine client intent (status or login)
3. Login state → authentication, profile fetching
4. Play state → upgrade to `JavaConnection`, add to world
5. Async packet tasks handle incoming/outgoing queues with compression & encryption
6. Server tick loop runs at 20 TPS, ticking all players and worlds

## Critical Patterns and Conventions

**Naming:**
- Use **Parchment mapping names** (vanilla Java naming) for consistency
- Packet prefixes: `C*` for clientbound (server→client), `S*` for serverbound (client→server)
- `*Reg` suffixes for registry types
- File-level doc comments with `//!` on most modules

**Concurrency:**
- `Arc<RwLock<T>>` / `Arc<Mutex<T>>` from `parking_lot` for shared mutable state
- Atomic types for lock-free reads (`AtomicBool`, `AtomicCell`)
- `TaskTracker` for graceful shutdown
- `CancellationToken` for cascading shutdown signals

**Registry System:**
- Global static `REGISTRY` using `OnceLock` (one-time initialization)
- Vanilla data loaded at startup via build-time generated code (~2MB of generated registries)
- Freeze mechanism prevents modifications after startup
- Query by `Identifier` (e.g., `minecraft:stone`)

**Packet Design:**
- Trait-based: `ServerPacket` (read from network), `ClientPacket` (write to network)
- Automatic ID assignment based on `ConnectionProtocol` state
- Compression/encryption at transport layer
- Handwritten packet structs for game packets (50+ files in steel-protocol/src/packets/)

**Player State:**
- Multi-threaded packet handling (separate incoming/outgoing tasks)
- Client-side state tracking (position, chunk view, loaded status)
- Message signature validation with expiry grace period
- Inventory syncing with state ID to detect desync

## Key Components

**Server (steel-core/src/server/mod.rs):**
- Central hub with cancellation token, multiple worlds, command dispatcher
- Main tick loop at 20 TPS with sprint/normal modes
- Manages graceful shutdown

**World (steel-core/src/world/mod.rs):**
- Contains `ChunkMap` and player HashMap
- Broadcasts messages and updates to all players

**Player (steel-core/src/player/mod.rs):**
- Core player entity with position, health, game mode, inventory
- `MessageChain` for chat signature validation
- `ChunkSender` for efficient chunk streaming
- Ticket system for chunk loading

**ChunkMap (steel-core/src/chunk/chunk_map.rs):**
- Manages chunk lifecycle, generation tasks, player chunk views
- Dirty tracking for modified chunks
- Dual runtime pattern: delegates generation to chunk runtime

**Chunk System:**
- `ProtoChunk` → `LevelChunk` hierarchy
- Paletted containers for efficient block storage (matching vanilla)
- Flat world generator currently implemented

**Network (steel/src/network/):**
- `JavaTcpClient` handles connection lifecycle through protocol states
- `JavaConnection` upgraded from TcpClient for Play state
- Async packet tasks for non-blocking I/O

## Important Implementation Notes

- **Consider future use cases**: When removing "unnecessary" vanilla elements, consider their potential future use. The project aims to match vanilla behavior closely.
- **Crate placement**: Think about modularity when choosing which crate to place new functionality in
- **Before major changes**: Discuss in [#feature-discussion](https://discord.gg/MwChEHnAbh) channel
- **Security**: Be careful with command injection, XSS in chat, and other vulnerabilities
- **Generated code**: Files in `generated/` directories are auto-generated; modify generation scripts instead
- **Message chain validation**: Full Minecraft 1.21 signed chat implementation is critical for online mode

## Current Development Status

**Implemented:**
- TCP networking with encryption/compression
- Handshake, Status, Login, Config protocol states
- Online mode authentication via Mojang session API
- Flat world generation
- Chunk loading and streaming to players
- Player inventory and equipment
- Signed chat with message chain validation
- Server tick loop with TPS management
- Command framework
- Creative mode slot setting

**In Progress:**
- Inventory improvements (containers, click handling)
- Player health/damage system
- Entity spawning and interactions
- Block interactions

**Not Yet Implemented:**
- Full entity system (mobs, passive entities)
- Most block behaviors
- Redstone mechanics
- Crafting/furnace logic beyond menus
- Physics (falling, fluids)
- World persistence (NBT save/load format)

## Getting Started for Contributors

1. Read the README.md and CONTRIBUTING.md files
2. Study the `Player` struct (steel-core/src/player/mod.rs) - central to understanding game state
3. Review `Server::run()` (steel-core/src/server/mod.rs) - the main game loop
4. Understand the packet flow from `JavaTcpClient` through handlers
5. Decompile Minecraft 1.21.11 using Parchment mappings for reference
6. Join the Discord server at https://discord.gg/MwChEHnAbh for questions

## References

- [Parchment Mappings Tutorial](https://parchmentmc.org/docs/getting-started)
- [GitCraft](https://github.com/WinPlay02/GitCraft) - Tool for decompiling Minecraft with mappings
- [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) - Sister project this is partially based on
