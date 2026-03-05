use std::{env, fs, path::Path, time::Duration};

use notify_debouncer_mini::*;

struct SolutionStats {
    cycles: u16,
    cost: u16,
    area: u16,
    sum: u16,
}

fn main() {
    let mut current_sol_stats = SolutionStats {
        cycles: 0,
        cost: 0,
        area: 0,
        sum: 0,
    };

    // get directory with solution files
    let base_path = env::var("USERPROFILE").unwrap() + r"\Documents\My Games\Opus Magnum\";
    let solution_dir = fs::read_dir(&base_path)
        .expect("Could not read Opus Magnum directory")
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .expect("Could not find a SteamID folder in Opus Magnum directory");

    // set up debounced file watcher to check changed *.solution files every second
    let mut latest_solution_debounce = new_debouncer(
        Duration::from_secs(1),
        |result: DebounceEventResult| match result {
            Ok(events) => events.iter().for_each(updateSolStats),
            Err(e) => println!("Error: {:?}", e),
        },
    )
    .unwrap();
    // point watcher to dir
    latest_solution_debounce
        .watcher()
        .watch(
            Path::new(&solution_dir),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();
    loop {}
}

fn updateSolStats(event: &DebouncedEvent) {
    let path = &event.path;
    if path.extension().is_none_or(|ext| ext != "solution") {
        return;
    };
}
