use log::info;

use super::command_queue::{Command, CommandQueue};

#[derive(Default)]
pub struct Field<'a> {
    command_queue: CommandQueue<'a>,
}

impl Field<'_> {
    pub fn new() -> Self {
        Self {
            command_queue: CommandQueue::default(),
        }
    }

    pub fn advance_queue(&mut self) -> Command {
        // TODO remove unwrap
        let next = self.command_queue.next().unwrap();
        info!("advance with: {:?}", next);

        next
    }
}

#[cfg(test)]
mod test {

    use super::Command;
    use super::Field;

    #[test]
    fn advance_queue() {
        let mut field = Field::new();

        assert_eq!(field.advance_queue(), Command::TurnRight);
        assert_eq!(field.advance_queue(), Command::TurnLeft);
        assert_eq!(field.advance_queue(), Command::TurnRight);
        assert_eq!(field.advance_queue(), Command::TurnLeft);
    }
}
