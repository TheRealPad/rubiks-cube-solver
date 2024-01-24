use crate::Face::Face::{CaseColor, FacePosition};

pub fn string_to_face_position(str: &String) -> FacePosition {
    match str.as_str() {
        "Front" => FacePosition::Front,
        "Back" => FacePosition::Back,
        "Left" => FacePosition::Left,
        "Right" => FacePosition::Right,
        "Top" => FacePosition::Top,
        "Down" => FacePosition::Down,
        _ => FacePosition::Down
    }
}

pub fn string_to_case_color(str: &String) -> CaseColor {
    match str.as_str() {
        "R" => CaseColor::Red,
        "G" => CaseColor::Green,
        "B" => CaseColor::Blue,
        "O" => CaseColor::Orange,
        "Y" => CaseColor::Yellow,
        "W" => CaseColor::White,
        _ => CaseColor::Red
    }
}