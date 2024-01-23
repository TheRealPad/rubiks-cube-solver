use crate::constants::movements::U;
use crate::Cube::Cube::Cube;
use crate::Face::Face::{CaseColor, Face, FacePosition};
use crate::Movements::Movements::Movements;
use crate::Solver::Solver::Solver;

pub struct Fridrich {
    movements: Option<Vec<String>>,
    cube: Option<Cube>
}

impl Solver for Fridrich {
    fn new() -> &'static mut dyn Solver {
        let fridrich = Box::leak(Box::new(Fridrich {
            movements: None,
            cube: None,
        }));
        fridrich as &'static mut dyn Solver
    }

    fn get_description(&self) -> String {
        return "The Fridrich method".to_string()
    }

    fn set_cube(&mut self, cube: Cube) {
        self.cube.replace(cube);
    }
    fn get_cube(& self) -> Cube {
        if self.cube.is_none() {
            return Cube::new(vec![
                Face::new(CaseColor::White, FacePosition::Top),
                Face::new(CaseColor::Yellow, FacePosition::Down),
                Face::new(CaseColor::Red, FacePosition::Left),
                Face::new(CaseColor::Blue, FacePosition::Front),
                Face::new(CaseColor::Green, FacePosition::Back),
                Face::new(CaseColor::Orange, FacePosition::Right)]);
        }
        self.cube.as_ref().cloned().unwrap()
    }

    fn get_number_of_movements(& self) -> usize {
        if self.movements.is_none() {
            return 0;
        }
        self.movements.clone().unwrap().len()
    }

    fn get_movements(& self) -> Vec<String> {
        if self.movements.is_none() {
            return vec![];
        }
        self.movements.clone().unwrap()
    }

    fn resolve(&mut self) {
        if let Some(mut cube) = self.cube.take() {
            let mut movements = self.movements.take().unwrap_or_else(|| Vec::new()); // Take ownership of self.movements or create an empty vector
            let mut movements_ref = Movements::new(&mut cube);
            movements_ref.call_movement(U);
            // Add more movements or resolve logic as needed
            movements.push(U.to_string());

            // Update self.cube and self.movements
            self.cube.replace(cube);
            self.movements.replace(movements);
        } else {
            // Handle the case when self.cube is None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use crate::Solver::Peripherals::PERIPHERALS;
    use super::*;

    #[test]
    fn test_update_cube() {
        let result_solver = panic::catch_unwind(|| unsafe { PERIPHERALS.take_serial("".to_string()) });

        if result_solver.is_err() {
            println!("cannot create 2 instances of Solver");
            return;
        }
        let mut solver = result_solver.unwrap();
        solver.set_cube(Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]));
        let cubeA = solver.get_cube();
        assert_eq!(cubeA.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(solver.get_movements().len(), 0);
        solver.resolve();
        let cubeB = solver.get_cube();
        assert_eq!(cubeB.get_face(FacePosition::Front).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(solver.get_movements().len(), 1);
    }
}
