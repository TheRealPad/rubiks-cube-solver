use crate::Cube::Cube::Cube;
use crate::Face::face::{CaseColor, Face, FacePosition};

pub fn rubiks_cube_solver() {
    let mut cube = Cube::new(vec![
        Face::new(CaseColor::White, FacePosition::Top),
        Face::new(CaseColor::Yellow, FacePosition::Down),
        Face::new(CaseColor::Red, FacePosition::Left),
        Face::new(CaseColor::Blue, FacePosition::Front),
        Face::new(CaseColor::Green, FacePosition::Back),
        Face::new(CaseColor::Orange, FacePosition::Right)]);

    let face = cube.get_face(FacePosition::Front);
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    println!("{:?}", face.get_face_position());
    let mut new_face = Face::new(CaseColor::Red, FacePosition::Front);
    new_face.set_line(1, (CaseColor::Blue, CaseColor::White, CaseColor::Red));
    cube.set_face(new_face);
    let face = cube.get_face(FacePosition::Front);
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    println!("{:?}", face.get_face_position());
}
