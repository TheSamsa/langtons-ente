use super::command_queue::Command;
use super::direction::Direction;
use super::position::Position;

#[derive(Default)]
pub struct Ant {
    position: Position,
    direction: Direction,
}

impl Ant {
    pub fn step(&mut self) -> Position {
        self.position.move_forward(self.direction)
    }

    pub fn execute_command(&mut self, command: Command) {
        self.direction = command.turn(&self.direction);
    }
}
