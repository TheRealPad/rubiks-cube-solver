use std::panic;
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
        println!("\t\t* fridrich (if cannot find the name, default method)");
        return true;
    }
    if args.len() < 3 {
        panic!("not enough parameters")
    }
    return false
}

pub fn rubiks_cube_solver(args: Vec<String>) {
    if error_handling(&args) {
        return;
    }
    let result_solver = panic::catch_unwind(|| unsafe { PERIPHERALS.take_serial(args.get(2).unwrap().to_string()) });

    if result_solver.is_err() {
        println!("cannot create 2 instances of Solver");
        return;
    }
    let solver = result_solver.unwrap();
    println!("{:?}", args);
    println!("{:?}", solver.get_description());
}
