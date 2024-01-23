use crate::Cube::Cube::Cube;

/// Interface Solver
pub trait Solver {
    fn new() -> &'static mut dyn Solver
        where
            Self: Sized;
    /// return the description of the selected solver
    fn get_description(& self) -> String;
    fn set_cube(&mut self, cube: Cube);
    fn get_cube(& self) -> Cube;
    fn get_number_of_movements(& self) -> usize;
    fn get_movements(& self) -> Vec<String>;
    fn resolve(&mut self);
}

#[cfg(test)]
mod tests {
    use crate::Solver::Fridrich::Fridrich::Fridrich;
    use super::*;

    #[test]
    fn test_example_solver_description() {
        let example_solver: &dyn Solver = Fridrich::new();
        let description = example_solver.get_description();

        assert_eq!(description, "The Fridrich method");
    }
}
