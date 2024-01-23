use std::ops::Deref;
use crate::Solver::Fridrich::Fridrich::{Fridrich};
use crate::Solver::Solver::Solver;

pub struct Peripherals {
    pub solver: Option<&'static mut dyn Solver>,
}

impl Peripherals {
    pub fn take_serial(&mut self, method: String) -> &'static mut dyn Solver {
        let mut solver: &'static mut dyn Solver = if method.eq("Fridrich") {
            Fridrich::new()
        } else {
            Fridrich::new()
        };

        // self.solver = Some(solver);
        solver
    }
}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    solver: None,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_serial_fridrich() {
        unsafe {
            PERIPHERALS.solver = None;
        }
        let solver = unsafe { PERIPHERALS.take_serial("Fridrich".to_string()) };

        assert_eq!(solver.get_description(), "The Fridrich method");
    }

    #[test]
    fn test_take_serial_default() {
        unsafe {
            PERIPHERALS.solver = None;
        }
        let solver = unsafe { PERIPHERALS.take_serial("unknown_method".to_string()) };

        assert_eq!(solver.get_description(), "The Fridrich method");
    }
}
