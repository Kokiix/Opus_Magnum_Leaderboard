use notify_debouncer_mini::*;
use opus_magnum_summer::{SolutionStats, get_steam_id_folder, parse_solution_stats};
use std::{env, fs, path::Path, time::Duration};

fn main() {
    let (steam_id, steam_id_folder) = get_steam_id_folder();
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
            Ok(events) => {
                for e in events {
                    update_maybe_upload_stats(&e, &mut current_sol_stats);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        },
    )
    .unwrap();

    latest_solution_debounce
        .watcher()
        .watch(
            Path::new(&steam_id_folder),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();

    // Watcher runs on separate thread, so we want to totally block this one
    loop {
        std::thread::sleep(Duration::from_millis(10000));
    }
}

fn update_maybe_upload_stats(event: &DebouncedEvent, stats: &mut SolutionStats) {
    let path = &event.path;
    if path.extension().is_none_or(|ext| ext != "solution") {
        return;
    };
    let filename = path.file_stem().unwrap().to_string_lossy().into_owned();

    if let Ok(data) = fs::read(path) {
        if let Some(mut new_stats) = parse_solution_stats(&data, filename)
        // && new_stats < *stats
        {
            // if new_stats.level != stats.level {
            //     *stats = new_stats;
            //     return;
            // }
            new_stats.steam_id = stats.steam_id.clone();
            *stats = new_stats;

            // Drop request if it fails for now
            let send_stats = || -> Result<(), ureq::Error> {
                let body = serde_json::to_string(stats).unwrap();
                ureq::post("https://opus-magnum-leaderboard.vercel.app/api/uploadSingleScore")
                    .header("Content-Type", "application/json")
                    .header(
                        // TODO: move into env file (and change key bc of commmit history)
                        "very_secret_key",
                        "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=",
                    )
                    .send(body)?;
                return Ok(());
            };
            if let Err(e) = send_stats() {
                eprintln!("Failed to send stats :(      {:?}", e);
            }
            // Debug print
            println!(
                "Updated stats for {}: Cycles: {}, Cost: {}, Area: {}",
                stats.level, stats.cycles, stats.cost, stats.area
            );
        }
    }
}
