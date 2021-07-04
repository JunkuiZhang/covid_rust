pub enum GameStatus {
    Paused,
    Running,
}

impl GameStatus {
    pub fn toggle(&mut self) {
        match *self {
            GameStatus::Paused => {
                *self = GameStatus::Running;
            }
            GameStatus::Running => {
                *self = GameStatus::Paused;
            }
        }
    }
}
