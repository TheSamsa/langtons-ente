#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Direction {
    pub horizontal: isize,
    pub vertical: isize,
}

impl Direction {
    pub fn new(horizontal: isize, vertical: isize) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            horizontal: 0,
            vertical: -1,
        }
    }
}
