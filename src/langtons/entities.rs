pub mod ant;
pub mod plane;
pub mod face;

#[derive(PartialEq, Eq, Hash)]
pub struct Position {
    horizontal: isize,
    vertical: isize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: isize::default(),
            vertical: isize::default(),
        }
    }
}

pub enum Command {
    TurnLeft,
    TurnRight,
    TurnDoubleLeft,
    TurnDoubleRight,
}

impl Command {
    pub fn new() -> Self {
        Self::TurnLeft
    }
}