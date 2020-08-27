#[derive(Default)]
pub(crate) struct Roll {
    difficulty: u8,
    success_count: u8,
    messy: bool,
    critical: bool,
}

impl Roll {
    fn new(difficulty: u8, success_count: u8, messy: bool, critical: bool) -> Self {
        Self {
            difficulty,
            success_count,
            messy,
            critical,
        }
    }
    fn is_success(&self) -> bool {
        self.success_count >= self.difficulty
    }
    fn is_messy_critical(&self) -> bool {
        self.messy && self.critical && self.is_success()
    }
    fn is_critical(&self) -> bool {
        self.critical && self.is_success() && !self.messy
    }
    fn is_failure(&self) -> bool {
        !self.is_success()
    }
    fn is_messy_critical_failure(&self) -> bool {
        self.messy && self.critical && self.is_failure()
    }
    fn is_critical_failure(&self) -> bool {
        self.critical && self.is_failure() && !self.messy
    }
}

pub(crate) fn roll(number_dies: u8, difficulty: u8, hunger: u8) -> Roll {
    Roll::default()
}
