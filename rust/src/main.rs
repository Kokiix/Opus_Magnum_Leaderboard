use notify_debouncer_mini::*;
use opus_magnum_summer::{SolutionStats, get_steam_id_folder, parse_solution_file, upload_stats};
use std::{env, fs, path::Path, time::Duration};

fn main() {
    let steam_id = get_steam_id_folder()
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let mut current_sol_stats = SolutionStats {
        cycles: 0,
        cost: 0,
        area: 0,
        sum: 0,
        level: "".to_string(),
        steam_id,
    };

    let mut latest_solution_debounce = new_debouncer(
        Duration::from_secs(1),
        move |result: DebounceEventResult| match result {
            Ok(e) => handle_events(e, &mut current_sol_stats),
            Err(e) => println!("Error: {:?}", e),
        },
    )
    .unwrap();

    latest_solution_debounce
        .watcher()
        .watch(&get_steam_id_folder(), notify::RecursiveMode::NonRecursive)
        .unwrap();

    // Watcher runs on separate thread, so we want to totally block this one
    loop {
        std::thread::sleep(Duration::from_millis(10000));
    }
}

fn handle_events(events: Vec<DebouncedEvent>, curr_stats: &mut SolutionStats) {
    for e in events {
        match parse_solution_file(&e.path) {
            None => continue,
            Some(new_stats) => {
                // if new_stats.level == curr_stats.level && new_stats.sum < curr_stats.sum {
                let _ = upload_stats(&new_stats);
                // }
                let id = curr_stats.steam_id.clone();
                *curr_stats = new_stats;
                curr_stats.steam_id = id;
            }
        }
    }
}
