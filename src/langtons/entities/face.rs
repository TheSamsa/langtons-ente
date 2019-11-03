
// use super::super::Face;
use super::Command;

pub struct Face {
    internal_angle: usize,
    // alternatively store the command to apply next
    // also could hold some arbitrary number and transcode it to colour or command
    colour: (usize, usize, usize),
}

impl Face {
    pub fn new() -> Self {
        Self {
            internal_angle: 90,
            colour: (255, 255, 255),
        }
    }
}
