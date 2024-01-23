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
