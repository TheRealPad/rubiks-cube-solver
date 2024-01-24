use std::panic;
use crate::Cube::Cube::Cube;
use crate::Face::Face::{CaseColor, Face, FacePosition};
use crate::lib::extract_json::{CubeFace, extract_json};
use crate::lib::read_file::get_file_content;
use crate::Solver::Peripherals::PERIPHERALS;
use crate::Solver::Solver::Solver;

fn error_handling(args: &Vec<String>) -> bool {
    if args.len() < 2 {
        panic!("not enough parameters")
    }
    if args.get(1).unwrap().contains("-h") || args.get(1).unwrap().contains("--help") {
        println!("DESCRIPTION:");
        println!("\tRubik's cube solver");
        println!("ARGUMENTS");
        println!("\t--a [name]: run the program with a selected algorithm");
        println!("\t\t* Fridrich (if cannot find the name, default method)");
        println!("\t--f [path]: path to the file containing the initial state of the cube");
        return true;
    }
    if args.len() < 5 {
        panic!("not enough parameters")
    }
    return false
}

fn extract_arguments(args: &[String]) -> (String, String) {
    let mut algorithm = "".to_string();
    let mut file_path = "".to_string();
    let mut iter = args.iter().enumerate().skip(1);

    while let Some(arg) = iter.next() {
        match arg.1.as_str() {
            "--a" => {
                if arg.0 < args.len() -1 {
                    algorithm = args.get(arg.0 + 1).unwrap().to_string()
                }
            },
            "--f" => {
                if arg.0 < args.len() -1 {
                    file_path = args.get(arg.0 + 1).unwrap().to_string()
                }
            },
            _ => {}
        }
    }
    (algorithm, file_path)
}

pub fn rubiks_cube_solver(args: Vec<String>) {
    if error_handling(&args) {
        return;
    }
    let (algorithm, file_path) = extract_arguments(&args);
    let v = extract_json::<Vec<CubeFace>>(get_file_content(file_path.to_string()).as_str());
    match v {
        Ok(_) => {}
        Err(e) => return
    }
    let result_solver = panic::catch_unwind(|| unsafe { PERIPHERALS.take_serial(algorithm) });

    if result_solver.is_err() {
        println!("cannot create 2 instances of Solver");
        return;
    }
    /* let mut solver = result_solver.unwrap();
    solver.set_cube(Cube::new(vec![
        Face::new(CaseColor::White, FacePosition::Top),
        Face::new(CaseColor::Yellow, FacePosition::Down),
        Face::new(CaseColor::Red, FacePosition::Left),
        Face::new(CaseColor::Blue, FacePosition::Front),
        Face::new(CaseColor::Green, FacePosition::Back),
        Face::new(CaseColor::Orange, FacePosition::Right)]));
    let cubeA = solver.get_cube();
    println!("{:?}", cubeA.get_face(FacePosition::Front));
    solver.resolve();
    let cubeB = solver.get_cube();
    println!("{:?}", cubeB.get_face(FacePosition::Front));
    println!("{:?}", solver.get_movements()); */
}
