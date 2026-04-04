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
