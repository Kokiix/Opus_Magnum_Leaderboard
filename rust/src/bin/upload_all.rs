use std::{collections::HashMap, env, fs};

use opus_magnum_summer::SolutionStats;

fn main() {
    let base_path = env::var("USERPROFILE").unwrap() + r"\Documents\My Games\Opus Magnum\";
    let steam_id_dir = fs::read_dir(&base_path)
        .expect("Opus Magnum dir reads properly")
        .next()
        .expect("steam_id dir exists")
        .expect("steam_id dir reads properly");
    let steam_id = steam_id_dir.file_name().to_string_lossy().into_owned();

    let best_scores: HashMap<String, SolutionStats> = HashMap::new();
    while let Ok(dir_entry) = fs::read_dir(&steam_id_dir.path()) {}
}
