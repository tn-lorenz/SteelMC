use std::{fs, path::Path};

mod banner_patterns;
mod biomes;
mod block_tags;
mod blocks;
mod cat_variants;
mod chat_types;
mod chicken_variants;
mod cow_variants;
mod damage_types;
mod dialogs;
mod dimension_types;
mod frog_variants;
mod instruments;
mod item_tags;
mod items;
mod jukebox_songs;
mod packets;
mod painting_variants;
mod pig_variants;
mod trim_materials;
mod trim_patterns;
mod wolf_sound_variants;
mod wolf_variants;

pub const OUT_DIR: &str = "src/generated";

pub fn main() {
    if !Path::new(OUT_DIR).exists() {
        fs::create_dir(OUT_DIR).unwrap();
    }

    let blocks = blocks::build().to_string();
    let block_tags = block_tags::build().to_string();
    let items = items::build().to_string();
    let item_tags = item_tags::build().to_string();
    let packets = packets::build().to_string();
    let banner_patterns = banner_patterns::build().to_string();
    let biomes = biomes::build().to_string();
    let chat_types = chat_types::build().to_string();
    let trim_patterns = trim_patterns::build().to_string();
    let trim_materials = trim_materials::build().to_string();
    let wolf_variants = wolf_variants::build().to_string();
    let wolf_sound_variants = wolf_sound_variants::build().to_string();
    let pig_variants = pig_variants::build().to_string();
    let frog_variants = frog_variants::build().to_string();
    let cat_variants = cat_variants::build().to_string();
    let cow_variants = cow_variants::build().to_string();
    let chicken_variants = chicken_variants::build().to_string();
    let painting_variants = painting_variants::build().to_string();
    let dimension_types = dimension_types::build().to_string();
    let damage_types = damage_types::build().to_string();
    let jukebox_songs = jukebox_songs::build().to_string();
    let instruments = instruments::build().to_string();
    let dialogs = dialogs::build().to_string();

    fs::write(format!("{}/vanilla_blocks.rs", OUT_DIR), blocks).unwrap();
    fs::write(format!("{}/vanilla_block_tags.rs", OUT_DIR), block_tags).unwrap();
    fs::write(format!("{}/vanilla_items.rs", OUT_DIR), items).unwrap();
    fs::write(format!("{}/vanilla_item_tags.rs", OUT_DIR), item_tags).unwrap();
    fs::write(format!("{}/packets.rs", OUT_DIR), packets).unwrap();
    fs::write(
        format!("{}/vanilla_banner_patterns.rs", OUT_DIR),
        banner_patterns,
    )
    .unwrap();
    fs::write(format!("{}/vanilla_biomes.rs", OUT_DIR), biomes).unwrap();
    fs::write(format!("{}/vanilla_chat_types.rs", OUT_DIR), chat_types).unwrap();
    fs::write(
        format!("{}/vanilla_trim_patterns.rs", OUT_DIR),
        trim_patterns,
    )
    .unwrap();
    fs::write(
        format!("{}/vanilla_trim_materials.rs", OUT_DIR),
        trim_materials,
    )
    .unwrap();
    fs::write(
        format!("{}/vanilla_wolf_variants.rs", OUT_DIR),
        wolf_variants,
    )
    .unwrap();
    fs::write(
        format!("{}/vanilla_wolf_sound_variants.rs", OUT_DIR),
        wolf_sound_variants,
    )
    .unwrap();
    fs::write(format!("{}/vanilla_pig_variants.rs", OUT_DIR), pig_variants).unwrap();
    fs::write(
        format!("{}/vanilla_frog_variants.rs", OUT_DIR),
        frog_variants,
    )
    .unwrap();
    fs::write(format!("{}/vanilla_cat_variants.rs", OUT_DIR), cat_variants).unwrap();
    fs::write(format!("{}/vanilla_cow_variants.rs", OUT_DIR), cow_variants).unwrap();
    fs::write(
        format!("{}/vanilla_chicken_variants.rs", OUT_DIR),
        chicken_variants,
    )
    .unwrap();
    fs::write(
        format!("{}/vanilla_painting_variants.rs", OUT_DIR),
        painting_variants,
    )
    .unwrap();
    fs::write(
        format!("{}/vanilla_dimension_types.rs", OUT_DIR),
        dimension_types,
    )
    .unwrap();
    fs::write(format!("{}/vanilla_damage_types.rs", OUT_DIR), damage_types).unwrap();
    fs::write(
        format!("{}/vanilla_jukebox_songs.rs", OUT_DIR),
        jukebox_songs,
    )
    .unwrap();
    fs::write(format!("{}/vanilla_instruments.rs", OUT_DIR), instruments).unwrap();
    fs::write(format!("{}/vanilla_dialogs.rs", OUT_DIR), dialogs).unwrap();
}
