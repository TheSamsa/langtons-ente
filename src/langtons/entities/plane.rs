use std::collections::HashMap;

// use super::super::EuclideanPlane;
// use super::super::Face;
use super::face::Face;
use super::Position;

pub struct Plane {
    plane_type: PlaneType,
    faces: HashMap<Position, Face>,
}

impl Plane {
    pub fn new() -> Self {
        let mut faces = HashMap::new();
        faces.insert(Position::new(), Face::new());

        Self {
            plane_type: PlaneType::Quadrille,
            faces,
        }
    }
    
    fn grab_or_create(&self, position: Position) {}

    fn append_or_overwrite(&mut self, face: Face, position: Position) {}
}

// struct Deltille {} Triangles
// struct Quadrille {} Quadrats
// struct Hextille {} Hexagonals
enum PlaneType {
    Deltille,
    Quadrille,
    Hextille,
}