use std::mem::replace;
use crate::Solver::fridrich::Fridrich::Fridrich;
use crate::Solver::roux::Roux::Roux;
use crate::Solver::Solver::Solver;

pub struct Peripherals {
    pub solver: Option<&'static dyn Solver>,
}

impl Peripherals {
    pub fn take_serial(&mut self, method: String) -> &'static dyn Solver {
        let solver: &'static dyn Solver = if method.eq("fridrich") {
            &Fridrich {}
        } else if method.eq("roux") {
            &Roux {}
        } else {
            &Fridrich {}
        };

        self.solver = Some(solver);
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
        let solver = unsafe { PERIPHERALS.take_serial("fridrich".to_string()) };

        assert_eq!(solver.get_description(), "The Fridrich method");
    }

    #[test]
    fn test_take_serial_roux() {
        unsafe {
            PERIPHERALS.solver = None;
        }
        let solver = unsafe { PERIPHERALS.take_serial("roux".to_string()) };

        assert_eq!(solver.get_description(), "The method roux");
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
