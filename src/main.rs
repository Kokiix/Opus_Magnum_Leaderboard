use std::{env, fs, time::Duration};

use notify_debouncer_mini::*;

fn main() {
    let mut latest_solution_debounce = new_debouncer(
        Duration::from_secs(1),
        |result: DebounceEventResult| match result {
            Ok(events) => {
                for event in events {
                    println!("{:?}", event);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        },
    )
    .unwrap();

    let base_path = env::var("USERPROFILE").unwrap() + r"\Documents\My Games\Opus Magnum\";
    let solution_dir = fs::read_dir(&base_path)
        .expect("Could not read Opus Magnum directory")
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .expect("Could not find a SteamID folder in Opus Magnum directory");

    println!("Watching directory: {:?}", solution_dir);

    latest_solution_debounce
        .watcher()
        .watch(
            Path::new(&solution_dir),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();

    loop {}
}
