use std::fs;

mod blocks;

pub const OUT_DIR: &str = "src/generated";

pub fn main() {
    let blocks = blocks::build().to_string();

    fs::write(format!("{}/vanilla_blocks.rs", OUT_DIR), blocks).unwrap();
}
