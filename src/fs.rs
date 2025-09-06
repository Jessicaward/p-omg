use std::fs;
use std::path::{Path, PathBuf};
use rand::{rng, Rng};

pub fn random_file(root: &str) -> Option<PathBuf> {
    let root_path = Path::new(root);
    let entries = fs::read_dir(root_path).ok()?;

    let mut rng = rng();
    let mut chosen: Option<PathBuf> = None;
    let mut count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            count += 1;
            if rng.gen_range(0..count) == 0 {
                chosen = Some(path);
            }
        }
    }

    chosen
}