use std::{env, fs, path::PathBuf};

use serde::Serialize;

#[derive(Serialize)]
pub struct SolutionStats {
    pub cycles: u16,
    pub cost: u16,
    pub area: u16,
    pub sum: u16,
    pub level: String,
    pub steam_id: String,
}

impl PartialEq for SolutionStats {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum && self.level == other.level
    }
}

impl PartialOrd for SolutionStats {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.sum.cmp(&other.sum))
    }
}

pub fn get_steam_id_folder() -> (String, PathBuf) {
    let base_path = env::var("USERPROFILE").unwrap() + r"\Documents\My Games\Opus Magnum\";
    let steam_id_dir = fs::read_dir(&base_path)
        .expect("Opus Magnum dir reads properly")
        .next()
        .expect("steam_id dir exists")
        .expect("steam_id dir reads properly");
    let steam_id = steam_id_dir.file_name().to_string_lossy().into_owned();

    return (steam_id, steam_id_dir.path());
}

pub fn parse_solution_file(path: &PathBuf) -> Option<SolutionStats> {
    if path.extension().is_none_or(|ext| ext != "solution") {
        return None;
    };
    let filename = path
        .file_stem()
        .expect("the file has a name")
        .to_string_lossy()
        .into_owned();
    let read_result = fs::read(path);
    if read_result.is_err() {
        return None;
    }
    let data = read_result.unwrap();

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
    skip_vlq_string(&data, &mut cursor)?;
    skip_vlq_string(&data, &mut cursor)?;

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
