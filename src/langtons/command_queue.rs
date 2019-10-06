use log::info;
use std::iter::Cycle;
use std::slice::Iter;

use super::direction::Direction;

pub struct CommandQueue<'a> {
    queue: Cycle<Iter<'a, Command>>,
}

impl Default for CommandQueue<'_> {
    fn default() -> Self {
        let queue = [Command::TurnRight, Command::TurnLeft].iter().cycle();

        Self { queue }
    }
}

impl Iterator for CommandQueue<'_> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.queue.next().unwrap();

        Some(next.to_owned())
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// enum Command
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    TurnRight,
    TurnLeft,
}

impl Command {
    pub fn turn(self, direction: &Direction) -> Direction {
        let mut new_direction = Direction::new(0, 0);
        match self {
            Self::TurnRight => {
                new_direction.horizontal = -direction.vertical;
                new_direction.vertical = direction.horizontal;
            }
            Self::TurnLeft => {
                new_direction.horizontal = direction.vertical;
                new_direction.vertical = -direction.horizontal;
            }
        }

        info!("turn from {:?} to {:?}", direction, new_direction);
        new_direction
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::TurnRight
    }
}

#[cfg(test)]
mod tests {

    use super::Command;
    use super::Direction;

    #[test]
    fn turns() {
        let direction_north = Direction::new(0, -1);
        let direction_east = Direction::new(1, 0);
        let direction_south = Direction::new(0, 1);
        let direction_west = Direction::new(-1, 0);

        assert_eq!(
            Command::TurnRight.turn(&direction_north),
            Direction::new(1, 0)
        );
        assert_eq!(
            Command::TurnRight.turn(&direction_east),
            Direction::new(0, 1)
        );
        assert_eq!(
            Command::TurnRight.turn(&direction_south),
            Direction::new(-1, 0)
        );
        assert_eq!(
            Command::TurnRight.turn(&direction_west),
            Direction::new(0, -1)
        );

        assert_eq!(
            Command::TurnLeft.turn(&direction_north),
            Direction::new(-1, 0)
        );
        assert_eq!(
            Command::TurnLeft.turn(&direction_east),
            Direction::new(0, -1)
        );
        assert_eq!(
            Command::TurnLeft.turn(&direction_south),
            Direction::new(1, 0)
        );
        assert_eq!(
            Command::TurnLeft.turn(&direction_west),
            Direction::new(0, 1)
        );
    }
}
