use crate::constants::movements::{B, B_REVERSE, D, D_DOUBLE, D_DOUBLE_REVERSE, D_DOUBLE_TWICE, D_REVERSE, F, F_REVERSE, L, L_DOUBLE, L_REVERSE, M, R, R_DOUBLE, R_REVERSE, U, U_DOUBLE, U_REVERSE, U_TWICE};
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

    move_cube.call_movement(U);
    move_cube.call_movement(B);
    move_cube.call_movement(B_REVERSE);
    let face = cube.get_face(FacePosition::Front);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    let face = cube.get_face(FacePosition::Left);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    let face = cube.get_face(FacePosition::Back);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    let face = cube.get_face(FacePosition::Right);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    let face = cube.get_face(FacePosition::Top);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
    let face = cube.get_face(FacePosition::Down);
    println!("{:?}", face.get_face_position());
    println!("{:?}", face.get_line(0));
    println!("{:?}", face.get_line(1));
    println!("{:?}", face.get_line(2));
}
