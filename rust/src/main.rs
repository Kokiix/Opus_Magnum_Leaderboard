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
            Ok(e) => handle_events(e, &mut current_sol_stats),
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

fn handle_events(events: Vec<DebouncedEvent>, curr_stats: &mut SolutionStats) {
    for e in events {
        match read_new_stats(&e, curr_stats) {
            None => continue,
            Some(new_stats) => {
                // if new_stats.level == curr_stats.level && new_stats.sum < curr_stats.sum {
                let _ = upload_stats(&new_stats);
                // }
                curr_stats.steam_id = new_stats.steam_id.clone();
                *curr_stats = new_stats;
            }
        }
    }
}

fn upload_stats(stats: &SolutionStats) -> Result<(), ureq::Error> {
    let body = serde_json::to_string(&stats).unwrap();
    ureq::post("https://opus-magnum-leaderboard.vercel.app/api/uploadSingleScore")
        .header("Content-Type", "application/json")
        .header(
            // TODO: move into env file (and change key bc of commmit history)
            "very_secret_key",
            "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=",
        )
        .send(body)?;

    // Debug print
    println!(
        "Updated stats for {}: Cycles: {}, Cost: {}, Area: {}",
        stats.level, stats.cycles, stats.cost, stats.area
    );

    return Ok(());
}

fn read_new_stats(event: &DebouncedEvent, stats: &mut SolutionStats) -> Option<SolutionStats> {
    let path = &event.path;
    if path.extension().is_none_or(|ext| ext != "solution") {
        return None;
    };
    let filename = path
        .file_stem()
        .expect("the file has a name")
        .to_string_lossy()
        .into_owned();

    if let Ok(data) = fs::read(path) {
        parse_solution_stats(&data, filename)
    } else {
        None
    }
}
