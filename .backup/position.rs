use log::info;

use super::direction::Direction;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Position {
    pub horizontal: isize,
    pub vertical: isize,
}

impl Position {
    pub fn move_forward(&mut self, direction: Direction) -> Self {
        self.horizontal += direction.horizontal;
        self.vertical += direction.vertical;

        info!("move to {:?}", self);

        self.clone()
    }
}
