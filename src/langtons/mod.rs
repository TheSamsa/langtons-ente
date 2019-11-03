
pub mod entities;

use entities::Position;
use entities::face::Face;
use entities::plane::Plane;
use entities::ant::Duck;

pub trait LangtonsAnt {
    fn issue_command(&mut self) -> Position;
    fn grab_or_create(&self) -> Face;
    fn append_or_overwrite(&mut self, face: Face, position: Position);
    fn step_forward(&mut self);
}

pub struct Langtons {
    plane: Plane,
    active_face: Face,
    duck: Duck,
}

impl Langtons {
    pub fn new() -> Self {
        Self {
            plane: Plane::new(),
            active_face: Face::new(),
            duck: Duck::new(),
        }
    }
}

impl LangtonsAnt for Langtons {
    fn issue_command(&mut self) ->Position { Position::new() }
    fn grab_or_create(&self) -> Face { Face::new() }
    fn append_or_overwrite(&mut self, face: Face, position: Position) {}
    fn step_forward(&mut self) {}
}
