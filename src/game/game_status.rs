pub enum GameStatus {
    Paused,
    Running,
}

impl GameStatus {
    pub fn toggle(&self) -> Self {
        match *self {
            GameStatus::Paused => {
                return GameStatus::Running;
            }
            GameStatus::Running => {
                return GameStatus::Paused;
            }
        }
    }
}
