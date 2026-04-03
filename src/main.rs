use notify_debouncer_mini::*;
use std::{env, fs, path::Path, time::Duration};

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
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for e in events {
                    update_sol_stats(&e, &mut current_sol_stats);
                }
            }
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

    loop {
        std::thread::sleep(Duration::from_millis(10000));
    }
}

fn update_sol_stats(event: &DebouncedEvent, stats: &mut SolutionStats) {
    let path = &event.path;
    if path.extension().is_none_or(|ext| ext != "solution") {
        return;
    };

    if let Ok(data) = fs::read(path) {
        if let Some(new_stats) = parse_solution_stats(&data) {
            *stats = new_stats;
            println!(
                "Updated stats for {}: Cycles: {}, Cost: {}, Area: {}",
                path.file_name().unwrap_or_default().to_string_lossy(),
                stats.cycles,
                stats.cost,
                stats.area
            );
        }
    }
}

fn parse_solution_stats(data: &[u8]) -> Option<SolutionStats> {
    let mut cursor = 0;

    // version number (4 bytes) == 7
    if data.len() < 4 || u32::from_le_bytes(data[0..4].try_into().ok()?) != 7 {
        return None;
    }
    cursor += 4;

    // TODO: only read the first bit
    // skip string names of the puzzle and the solution file
    fn skip_vlq_string(data: &[u8], cursor: &mut usize) -> Option<()> {
        let mut len: usize = 0;
        let mut shift = 0;
        loop {
            let byte = *data.get(*cursor)?;
            *cursor += 1;
            len |= ((byte & 0x7F) as usize) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        *cursor += len;
        if *cursor > data.len() {
            return None;
        }
        Some(())
    }
    skip_vlq_string(data, &mut cursor)?;
    skip_vlq_string(data, &mut cursor)?;

    // 4. Solved Flag (4 bytes)
    if cursor + 4 > data.len() {
        return None;
    }
    let solved = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?);
    cursor += 4;

    if solved == 0 {
        return None;
    }

    // 5. Extract Metrics (Checker Block)
    // Markers (0, 1, 2, 3) are interleaved between data.
    // 4 bytes: Marker 0, 4 bytes: Cycles, 4 bytes: Marker 1, 4 bytes: Cost, 4 bytes: Marker 2, 4 bytes: Area
    // (There is also Marker 3 and Instructions, but we stop at Area)
    if cursor + 24 > data.len() {
        return None;
    }

    cursor += 4; // Skip Marker 0
    let cycles = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;
    cursor += 4;

    cursor += 4; // Skip Marker 1
    let cost = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;
    cursor += 4;

    cursor += 4; // Skip Marker 2
    let area = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;
    // ... we could read instructions here if we wanted

    Some(SolutionStats {
        cycles,
        cost,
        area,
        sum: cycles + cost + area,
    })
}
