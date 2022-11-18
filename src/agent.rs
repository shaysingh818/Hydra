#[derive(Debug, Copy, Clone)]
pub struct Agent {
    uuid: i32,
    score: i32,
    active: bool,
    history: i32,
}

impl Agent {
    pub fn new(set_uuid: i32) -> Agent {
        Agent {
            uuid: set_uuid,
            score: 0,
            history: 0,
            active: false,
        }
    }

    pub fn get_status(&self) -> bool {
        self.active
    }

    pub fn get_piece(&self) -> i32 {
        self.uuid
    }

    pub fn get_history(&self) -> i32 {
        self.history
    }

    pub fn set_score(&mut self, num: i32) {
        self.score = num;
    }

    pub fn set_status(&mut self, status: bool) {
        self.active = status;
    }
}
