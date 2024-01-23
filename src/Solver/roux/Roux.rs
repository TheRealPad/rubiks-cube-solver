use crate::Solver::Solver::Solver;

pub struct Roux {
    
}

impl Solver for Roux {
    fn get_description(&self) -> String {
        return "The method roux".to_string();
    }
}