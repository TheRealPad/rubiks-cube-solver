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
pub struct Face {
    face: FacePosition,
    cases: Vec<Vec<CaseColor>>
}

impl Face {
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

    pub fn get_line(& self, pos: usize)-> (CaseColor, CaseColor, CaseColor) {
        (self.cases[pos][0].clone(), self.cases[pos][1].clone(), self.cases[pos][2].clone())
    }

    pub fn get_column(& self, pos: usize)-> (CaseColor, CaseColor, CaseColor) {
        (self.cases[0][pos].clone(), self.cases[1][pos].clone(), self.cases[2][pos].clone())
    }

    pub fn set_column(&mut self, pos: usize, colors: (CaseColor,CaseColor,CaseColor)) {
        self.cases[0][pos] = colors.0.clone();
        self.cases[1][pos] = colors.1.clone();
        self.cases[2][pos] = colors.2.clone();
    }

    pub fn set_line(&mut self, pos: usize, colors: (CaseColor,CaseColor,CaseColor)) {
        self.cases[pos][0] = colors.0.clone();
        self.cases[pos][1] = colors.1.clone();
        self.cases[pos][2] = colors.2.clone();
    }

    pub fn get_face_position(& self) -> FacePosition {
        self.face.clone()
    }

    pub fn set_face_position(&mut self, face: FacePosition) {
        self.face = face
    }
}

#[cfg(test)]
mod tests {
    use crate::Face::face::{CaseColor, Face, FacePosition};

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
