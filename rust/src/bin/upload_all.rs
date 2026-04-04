use std::{collections::HashMap, env, fs};

use opus_magnum_summer::{SolutionStats, get_steam_id_folder, parse_solution_file, upload_stats};

fn main() {
    let steam_id_folder = get_steam_id_folder();
    let mut best_scores: HashMap<String, SolutionStats> = HashMap::new();
    let steam_id_folder_obj = fs::read_dir(&steam_id_folder).expect("no read errors");
    for dir_entry in steam_id_folder_obj.flatten() {
        if let Some(stats) = parse_solution_file(&dir_entry.path()) {
            match best_scores.get_mut(&stats.level) {
                Some(old_best) => {
                    if stats.sum < old_best.sum {
                        *old_best = stats;
                    }
                }
                None => {
                    best_scores.insert(stats.level.clone(), stats);
                }
            };
        }
    }

    for solution in best_scores.values() {
        let _ = upload_stats(solution);
    }
}
