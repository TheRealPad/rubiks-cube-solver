use crate::Solver::Solver::Solver;

pub struct Fridrich {
    
}

impl Solver for Fridrich {
    fn get_description(&self) -> String {
        return "The Fridrich method".to_string()
    }
}
