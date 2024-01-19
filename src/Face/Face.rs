#![crate_name = "doc"]

#[derive(Debug, Clone, PartialEq)]
pub enum CaseColor {
    Red,
    Orange,
    Blue,
    Green,
    White,
    Yellow
}

#[derive(Debug, PartialEq, Clone)]
pub enum FacePosition {
    Front,
    Back,
    Top,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
/// One face of the cube is represented here
pub struct Face {
    /// the position of the face, see FacePosition
    face: FacePosition,
    /// a vector for all the case of the rubi's cube, a 3x3 has 9 cases
    cases: Vec<Vec<CaseColor>>
}

impl Face {
    /// Returns a face with the face and color cases given
    ///
    /// # Arguments
    ///
    /// * `color` - The CaseColor enum for all the cases
    /// * `face` - The FacePosition
    ///
    /// # Examples
    ///
    /// ```
    /// Face::new(CaseColor::Red, FacePosition::Front);
    /// ```
    pub fn new(color: CaseColor, face: FacePosition) -> Face {
        let mut cases = Vec::with_capacity(3);

        for _ in 0..3 {
            let row = vec![color.clone(); 3];
            cases.push(row);
        }
        Face {
            face,
            cases,
        }
    }

    /// return the 3 case colors from the given line
    ///
    /// # Arguments
    ///
    /// * `pos` - the pos of the line, between 0 and 2
    ///
    pub fn get_line(& self, pos: usize)-> (CaseColor, CaseColor, CaseColor) {
        (self.cases[pos][0].clone(), self.cases[pos][1].clone(), self.cases[pos][2].clone())
    }

    /// return the 3 case colors from the given column
    ///
    /// # Arguments
    ///
    /// * `pos` - the pos of the column, between 0 and 2
    ///
    pub fn get_column(& self, pos: usize)-> (CaseColor, CaseColor, CaseColor) {
        (self.cases[0][pos].clone(), self.cases[1][pos].clone(), self.cases[2][pos].clone())
    }

    /// set the new column's colors
    ///
    /// # Arguments
    ///
    /// * `pos` - the pos of the column, between 0 and 2
    /// * `colors` - the color for each cases, from top to bottom
    ///
    pub fn set_column(&mut self, pos: usize, colors: (CaseColor,CaseColor,CaseColor)) {
        self.cases[0][pos] = colors.0.clone();
        self.cases[1][pos] = colors.1.clone();
        self.cases[2][pos] = colors.2.clone();
    }

    /// set the new colors for a given line
    ///
    /// # Arguments
    ///
    /// * `pos` - the pos of the line, between 0 and 2
    /// * `colors` - 3 colors, for each case, from left to right
    ///
    pub fn set_line(&mut self, pos: usize, colors: (CaseColor,CaseColor,CaseColor)) {
        self.cases[pos][0] = colors.0.clone();
        self.cases[pos][1] = colors.1.clone();
        self.cases[pos][2] = colors.2.clone();
    }

    /// return the position of the face
    ///
    ///
    pub fn get_face_position(& self) -> FacePosition {
        self.face.clone()
    }

    /// set a new position for the face
    ///
    /// # Arguments
    ///
    /// * `face` - The new selected face
    ///
    pub fn set_face_position(&mut self, face: FacePosition) {
        self.face = face
    }
}

#[cfg(test)]
mod tests {
    use crate::Face::Face::{CaseColor, Face, FacePosition};

    #[test]
    fn test_initialization() {
        let face = Face::new(CaseColor::Red, FacePosition::Front);
        assert_eq!(face.get_face_position(), FacePosition::Front);
        assert_eq!(face.get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(face.get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(face.get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_ne!(face.get_line(2), (CaseColor::White, CaseColor::Red, CaseColor::Red))
    }

    #[test]
    fn test_change_position() {
        let mut face = Face::new(CaseColor::Red, FacePosition::Front);
        assert_eq!(face.get_face_position(), FacePosition::Front);
        face.set_face_position(FacePosition::Back);
        assert_eq!(face.get_face_position(), FacePosition::Back);
    }

    #[test]
    fn test_change_color() {
        let mut face = Face::new(CaseColor::Red, FacePosition::Front);
        assert_eq!(face.get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        face.set_line(0, (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(face.get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_ne!(face.get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
    }
}
