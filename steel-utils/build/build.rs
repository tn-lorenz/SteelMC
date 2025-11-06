use std::{fs, path::Path, process::Command};

mod translations;

const FMT: bool = true;

const OUT_DIR: &str = "src/generated";
const TRANSLATIONS: &str = "vanilla_translations";

pub fn main() {
    if !Path::new(OUT_DIR).exists() {
        fs::create_dir_all(OUT_DIR).unwrap();
    }

    let content = translations::build();
    fs::write(format!("{OUT_DIR}/{TRANSLATIONS}.rs"), content.to_string()).unwrap();

    if FMT && let Ok(entries) = fs::read_dir(OUT_DIR) {
        for entry in entries.flatten() {
            let _ = Command::new("rustfmt").arg(entry.path()).output();
        }
    }
}
