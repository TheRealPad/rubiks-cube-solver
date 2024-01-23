use crate::Cube::Cube::Cube;

/// Interface Solver
pub trait Solver {

    /// Create a new Solver instance
    fn new() -> &'static mut dyn Solver
        where
            Self: Sized;
    
    /// return the description of the selected solver
    fn get_description(& self) -> String;

    /// set a new cube to solve
    ///
    /// # Argument
    ///
    /// * `cube` - the cube to solve
    ///
    fn set_cube(&mut self, cube: Cube);

    /// get the actual state of the cube to solve
    ///
    /// # Return
    ///
    /// * `cube` - a copy of the cube
    ///
    fn get_cube(& self) -> Cube;

    /// get the number of movements used to solve the cube
    ///
    /// # Return
    ///
    /// * `usize` - a positive number
    ///
    fn get_number_of_movements(& self) -> usize;

    /// ge the vector containing all the movements used
    ///
    /// # Return
    ///
    /// * `Vec<String>` - a vector containing all the movements used
    ///
    fn get_movements(& self) -> Vec<String>;

    /// resolve the cube
    ///
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
