use crate::Cube::Cube::Cube;
use crate::Face::face::{CaseColor, Face, FacePosition};
use crate::Movements::Movements::Movements;

pub fn rubiks_cube_solver() {
    let mut cube = Cube::new(vec![
        Face::new(CaseColor::White, FacePosition::Top),
        Face::new(CaseColor::Yellow, FacePosition::Down),
        Face::new(CaseColor::Red, FacePosition::Left),
        Face::new(CaseColor::Blue, FacePosition::Front),
        Face::new(CaseColor::Green, FacePosition::Back),
        Face::new(CaseColor::Orange, FacePosition::Right)]);
    let mut move_cube = Movements::new(&mut cube);

    move_cube.move_u(false);
    let face = cube.get_face(FacePosition::Front);
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
}
