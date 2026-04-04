use std::{collections::HashMap, env, fs};

use opus_magnum_summer::{SolutionStats, get_steam_id_folder};

fn main() {
    let (steam_id, steam_id_folder) = get_steam_id_folder();

    let best_scores: HashMap<String, SolutionStats> = HashMap::new();
    while let Ok(dir_entry) = fs::read_dir(&steam_id_folder) {}
}
