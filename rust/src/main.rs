use notify_debouncer_mini::*;
use opus_magnum_summer::{SolutionStats, get_steam_id_folder};
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
            Path::new(&steam_id_folder),
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

    let level = filename.rsplit_once('-').unwrap().0.to_string();
    Some(SolutionStats {
        cycles,
        cost,
        area,
        sum: cycles + cost + area,
        level,
        steam_id: "".to_string(),
    })
}
