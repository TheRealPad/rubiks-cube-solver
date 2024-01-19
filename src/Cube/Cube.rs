use crate::Face::Face::{Face, FacePosition};

/// A Cube is represented here
pub struct Cube {
    /// a vector to contain the 6 faces
    faces: Vec<Face>,
}

impl Cube {
    /// Returns a cube with the given faces
    ///
    /// # Arguments
    ///
    /// * `cube` - vector containing the facing, must be of length 6
    ///
    /// # Examples
    ///
    /// ```
    /// Cube::new(vec![
    ///     Face::new(CaseColor::White, FacePosition::Top),
    ///     Face::new(CaseColor::Yellow, FacePosition::Down),
    ///     Face::new(CaseColor::Red, FacePosition::Left),
    ///     Face::new(CaseColor::Blue, FacePosition::Front),
    ///     Face::new(CaseColor::Green, FacePosition::Back),
    ///     Face::new(CaseColor::Orange, FacePosition::Right)]);
    /// ```
    pub fn new(cube: Vec<Face>) -> Cube {
        let mut faces = Vec::with_capacity(6);
        for face in cube {
            faces.push(face);
        }

        Cube {
            faces
        }
    }

    /// return the selected face
    ///
    /// # Arguments
    ///
    /// * `face` - the pos of the face
    ///
    pub fn get_face(& self, face: FacePosition) -> Face {
        if let Some(found_face) = self.faces.iter().find(|&f| f.get_face_position() == face) {
            found_face.clone()
        } else {
            self.faces[0].clone()
        }
    }

    /// set a new face, change the one that match the same FacePosition
    ///
    /// # Arguments
    ///
    /// * `face` - the new face
    ///
    pub fn set_face(&mut self, face: Face) {
        if let Some(mut found_face) = self.faces.iter_mut().find(|f| f.get_face_position() == face.get_face_position()) {
            found_face.set_face_position(face.get_face_position().clone());
            for i in 0..3 {
                found_face.set_line(i, face.get_line(i));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cube::Cube::Cube;
    use crate::Face::face::{CaseColor, Face, FacePosition};

    #[test]
    fn test_initialization() {
        let cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue))
    }

    #[test]
    fn test_change_face_one() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        let face = Face::new(CaseColor::Red, FacePosition::Front);
        cube.set_face(face);
        assert_ne!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
    }

    #[test]
    fn test_change_face_two() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        let mut face = Face::new(CaseColor::Red, FacePosition::Front);
        face.set_line(0, (CaseColor::Green, CaseColor::Yellow, CaseColor::Orange));
        cube.set_face(face);
        assert_ne!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Green, CaseColor::Yellow, CaseColor::Orange));
    }
}
