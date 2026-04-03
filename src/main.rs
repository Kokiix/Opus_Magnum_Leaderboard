use notify_debouncer_mini::*;
use std::{env, fs, path::Path, time::Duration};

struct SolutionStats {
    cycles: u16,
    cost: u16,
    area: u16,
    sum: u16,
    filename: String,
}

impl PartialEq for SolutionStats {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum && self.filename == other.filename
    }
}

fn main() {
    let mut current_sol_stats = SolutionStats {
        cycles: 0,
        cost: 0,
        area: 0,
        sum: 0,
        filename: "".to_string(),
    };

    let base_path = env::var("USERPROFILE").unwrap() + r"\Documents\My Games\Opus Magnum\";
    let solution_dir = fs::read_dir(&base_path)
        .expect("Could not read Opus Magnum directory")
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .expect("Could not find a SteamID folder in Opus Magnum directory");

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

    latest_solution_debounce
        .watcher()
        .watch(
            Path::new(&solution_dir),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();

    // Watcher runs on separate thread, so we want to totally block this one
    loop {
        std::thread::sleep(Duration::from_millis(10000));
    }
}

fn update_sol_stats(event: &DebouncedEvent, stats: &mut SolutionStats) {
    let path = &event.path;
    if path.extension().is_none_or(|ext| ext != "solution") {
        return;
    };
    let filename = path.file_stem().unwrap().to_string_lossy().into_owned();

    if let Ok(data) = fs::read(path) {
        if let Some(new_stats) = parse_solution_stats(&data, filename)
            && new_stats != *stats
            && new_stats.filename == *stats.filename
        {
            *stats = new_stats;

            // Debug print
            // println!(
            //     "Updated stats for {}: Cycles: {}, Cost: {}, Area: {}",
            //     path.file_name().unwrap_or_default().to_string_lossy(),
            //     stats.cycles,
            //     stats.cost,
            //     stats.area
            // );
        }
    }
}

fn parse_solution_stats(data: &[u8], filename: String) -> Option<SolutionStats> {
    let mut cursor = 0;

    // Ensure version number (4 bytes) == 7
    if data.len() < 4 || u32::from_le_bytes(data[0..4].try_into().ok()?) != 7 {
        return None;
    }
    cursor += 4;

    // AI-generated, likely not totally efficient
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

    if cursor + 28 > data.len() {
        return None;
    }

    let is_solved = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?);
    if is_solved == 0 {
        return None;
    }
    cursor += 4;

    // 4 byte markers interleaved between data
    cursor += 4;
    let cycles = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;
    cursor += 8;
    let cost = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;
    cursor += 8;
    let area = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as u16;

    Some(SolutionStats {
        cycles,
        cost,
        area,
        sum: cycles + cost + area,
        filename,
    })
}
