use std::collections::HashMap;
use steel_utils::ResourceLocation;

use crate::RegistryExt;

/// Represents a jukebox song definition from a data pack JSON file.
#[derive(Debug)]
pub struct JukeboxSong {
    pub key: ResourceLocation,
    pub sound_event: ResourceLocation,
    pub description: TextComponent,
    pub length_in_seconds: f32,
    pub comparator_output: i32,
}

/// A simplified representation of a translatable text component.
#[derive(Debug)]
pub struct TextComponent {
    pub translate: &'static str,
}

pub type JukeboxSongRef = &'static JukeboxSong;

pub struct JukeboxSongRegistry {
    jukebox_songs: HashMap<ResourceLocation, JukeboxSongRef>,
    allows_registering: bool,
}

impl JukeboxSongRegistry {
    pub fn new() -> Self {
        Self {
            jukebox_songs: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, jukebox_song: JukeboxSongRef) {
        if !self.allows_registering {
            panic!("Cannot register jukebox songs after the registry has been frozen");
        }

        self.jukebox_songs
            .insert(jukebox_song.key.clone(), jukebox_song);
    }
}

impl RegistryExt for JukeboxSongRegistry {
    fn freeze(&mut self) {
        self.allows_registering = false;
    }
}
