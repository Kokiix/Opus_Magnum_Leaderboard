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
