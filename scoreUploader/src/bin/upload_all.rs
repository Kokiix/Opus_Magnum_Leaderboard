use std::{collections::HashMap, fs, io};

use om_score_tracker::{SolutionStats, get_steam_id_folder, parse_solution_file};
use serde::Serialize;

#[derive(Serialize)]
struct BatchUpload {
    stats: Vec<SolutionStats>,
}

fn main() {
    let steam_id_folder = get_steam_id_folder();
    let steam_id = get_steam_id_folder()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let mut best_scores: HashMap<String, SolutionStats> = HashMap::new();
    println!("Beginning to scan files.");
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

    let mut batch = BatchUpload {
        stats: best_scores.into_values().collect(),
    };
    println!("Scanned {} files.", batch.stats.len());

    batch
        .stats
        .iter_mut()
        .for_each(|stats| stats.steam_id = steam_id.clone());
    let body = serde_json::to_string(&batch).unwrap();
    println!("Beginning Upload!");
    ureq::post("https://opus-magnum-leaderboard.vercel.app/api/uploadMultiScores")
        .header("Content-Type", "application/json")
        .header(
            // TODO: move into env file (and change key bc of commmit history)
            "very_secret_key",
            "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=",
        )
        .send(body)
        .expect("uploaded successfully");
    println!("Upload Success! Press Enter to exit.");
    let _ = io::stdin().read_line(&mut String::new());
}
