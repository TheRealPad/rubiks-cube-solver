/// Interface Solver
pub trait Solver {
    /// return the description of the selected solver
    fn get_description(& self) -> String;
}

#[cfg(test)]
mod tests {
    use crate::Solver::fridrich::Fridrich::Fridrich;
    use super::*;

    #[test]
    fn test_example_solver_description() {
        let example_solver: &dyn Solver = &Fridrich {};
        let description = example_solver.get_description();

        assert_eq!(description, "The Fridrich method");
    }
}
