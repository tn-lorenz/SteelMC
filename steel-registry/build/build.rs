use std::{fs, path::Path};

mod blocks;
mod items;

pub const OUT_DIR: &str = "src/generated";

pub fn main() {
    if !Path::new(OUT_DIR).exists() {
        fs::create_dir(OUT_DIR).unwrap();
    }

    let blocks = blocks::build().to_string();
    let items = items::build().to_string();

    fs::write(format!("{}/vanilla_blocks.rs", OUT_DIR), blocks).unwrap();
    fs::write(format!("{}/vanilla_items.rs", OUT_DIR), items).unwrap();
}
