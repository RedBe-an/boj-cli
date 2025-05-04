#[derive(Debug)]
pub enum Level {
    Unrated,
    Bronze(u8),   // 1 ~ 5
    Silver(u8),   // 1 ~ 5
    Gold(u8),     // 1 ~ 5
    Platinum(u8), // 1 ~ 5
    Diamond(u8),  // 1 ~ 5
    Ruby(u8),     // 1 ~ 5
}

impl Level {
    pub fn from_rank(rank: usize) -> Self {
        match rank {
            0 => Level::Unrated,
            1..=5 => Level::Bronze(rank as u8),
            6..=10 => Level::Silver((rank - 5) as u8),
            11..=15 => Level::Gold((rank - 10) as u8),
            16..=20 => Level::Platinum((rank - 15) as u8),
            21..=25 => Level::Diamond((rank - 20) as u8),
            26..=30 => Level::Ruby((rank - 25) as u8),
            _ => Level::Unrated,
        }
    }
}
