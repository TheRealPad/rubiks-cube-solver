use std::collections::HashMap;
use crate::constants::movements::{B, B_REVERSE, B_TWICE, D, D_DOUBLE, D_DOUBLE_REVERSE, D_DOUBLE_TWICE, D_REVERSE, D_TO_F, D_TWICE, F, F_REVERSE, F_TWICE, L, L_DOUBLE, L_DOUBLE_REVERSE, L_DOUBLE_TWICE, L_REVERSE, L_TO_F, L_TO_U, L_TWICE, M, M_REVERSE, M_TWICE, R, R_DOUBLE, R_DOUBLE_REVERSE, R_DOUBLE_TWICE, R_REVERSE, R_TO_F, R_TO_U, R_TWICE, U, U_DOUBLE, U_DOUBLE_REVERSE, U_DOUBLE_TWICE, U_REVERSE, U_TO_F, U_TWICE};
use crate::Cube::Cube::Cube;
use crate::Face::Face::{CaseColor, Face, FacePosition};

// A Movement is represented here
pub struct Movements<'a> {
    /// a ref to the cube we want to update
    cube: &'a mut Cube,
    /// a function to redirect to all the move functions
    move_functions: HashMap<&'static str, fn(&mut Movements<'a>, bool)>,
}

impl<'a> Movements<'a> {
    /// Returns a Movement with all the function to move
    ///
    /// # Arguments
    ///
    /// * `cube` - the ref to the cube
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cube = Cube::new(vec![
    ///     Face::new(CaseColor::White, FacePosition::Top),
    ///     Face::new(CaseColor::Yellow, FacePosition::Down),
    ///     Face::new(CaseColor::Red, FacePosition::Left),
    ///     Face::new(CaseColor::Blue, FacePosition::Front),
    ///     Face::new(CaseColor::Green, FacePosition::Back),
    ///     Face::new(CaseColor::Orange, FacePosition::Right)]);
    /// let mut move_cube = Movements::new(&mut cube);
    /// ```
    pub fn new(cube: &'a mut Cube) -> Movements<'a> {
        let mut move_functions: HashMap<&'static str, fn(&mut Movements<'a>, bool)> = HashMap::new();
        move_functions.insert(U, Self::move_u);
        move_functions.insert(U_REVERSE, Self::move_u);
        move_functions.insert(U_TWICE, Self::move_u);
        move_functions.insert(U_DOUBLE, Self::move_double_u);
        move_functions.insert(U_DOUBLE_REVERSE, Self::move_double_u);
        move_functions.insert(U_DOUBLE_TWICE, Self::move_double_u);
        move_functions.insert(R, Self::move_r);
        move_functions.insert(R_REVERSE, Self::move_r);
        move_functions.insert(R_TWICE, Self::move_r);
        move_functions.insert(R_DOUBLE, Self::move_double_r);
        move_functions.insert(R_DOUBLE_REVERSE, Self::move_double_r);
        move_functions.insert(R_DOUBLE_TWICE, Self::move_double_r);
        move_functions.insert(D, Self::move_d);
        move_functions.insert(D_REVERSE, Self::move_d);
        move_functions.insert(D_TWICE, Self::move_d);
        move_functions.insert(D_DOUBLE, Self::move_double_d);
        move_functions.insert(D_DOUBLE_REVERSE, Self::move_double_d);
        move_functions.insert(D_DOUBLE_TWICE, Self::move_double_d);
        move_functions.insert(L, Self::move_l);
        move_functions.insert(L_REVERSE, Self::move_l);
        move_functions.insert(L_TWICE, Self::move_l);
        move_functions.insert(L_DOUBLE, Self::move_double_l);
        move_functions.insert(L_DOUBLE_REVERSE, Self::move_double_l);
        move_functions.insert(L_DOUBLE_TWICE, Self::move_double_l);
        move_functions.insert(F, Self::move_f);
        move_functions.insert(F_REVERSE, Self::move_f);
        move_functions.insert(F_TWICE, Self::move_f);
        move_functions.insert(B, Self::move_b);
        move_functions.insert(B_REVERSE, Self::move_b);
        move_functions.insert(B_TWICE, Self::move_b);
        move_functions.insert(M, Self::move_m);
        move_functions.insert(M_REVERSE, Self::move_m);
        move_functions.insert(M_TWICE, Self::move_m);
        move_functions.insert(R_TO_F, Self::translate_r_to_f);
        move_functions.insert(L_TO_F, Self::translate_l_to_f);
        move_functions.insert(U_TO_F, Self::translate_u_to_f);
        move_functions.insert(D_TO_F, Self::translate_d_to_f);
        move_functions.insert(R_TO_U, Self::translate_r_to_u);
        move_functions.insert(L_TO_U, Self::translate_l_to_u);

        Movements { cube, move_functions }
    }

    /// take a string and call the matching move
    ///
    /// # Argument
    ///
    /// * `movement` - the string matching the move, see src/constants/movements.rs to see all movements
    ///
    pub fn call_movement(&mut self, movement: &str) {
        if let Some(func) = self.move_functions.get(movement) {
            func(self, movement.contains("'"));
        } else {
            println!("Unknown move: {}", movement);
        }
        if movement.contains("2") {
            if let Some(func) = self.move_functions.get(movement) {
                func(self, movement.contains("'"));
            } else {
                println!("Unknown move: {}", movement);
            }
        }
    }

    fn move_u(&mut self, reverse: bool) {
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut back = self.cube.get_face(FacePosition::Back);
        let mut right = self.cube.get_face(FacePosition::Right);
        let mut top = self.cube.get_face(FacePosition::Top);
        let saved_top = self.cube.get_face(FacePosition::Top);

        if reverse {
            let save_state = front.get_line(0);
            front.set_line(0, left.get_line(0));
            left.set_line(0, back.get_line(0));
            back.set_line(0, right.get_line(0));
            right.set_line(0, save_state);
            top.set_column(0, (saved_top.get_line(0).2, saved_top.get_line(0).1, saved_top.get_line(0).0));
            top.set_column(1, (saved_top.get_line(1).2, saved_top.get_line(1).1, saved_top.get_line(1).0));
            top.set_column(2, (saved_top.get_line(2).2, saved_top.get_line(2).1, saved_top.get_line(2).0));
        } else {
            let save_state = front.get_line(0);
            front.set_line(0, right.get_line(0));
            right.set_line(0, back.get_line(0));
            back.set_line(0, left.get_line(0));
            left.set_line(0, save_state);
            top.set_column(0, saved_top.get_line(2));
            top.set_column(1, saved_top.get_line(1));
            top.set_column(2, saved_top.get_line(0));
        }
        self.cube.set_face(front);
        self.cube.set_face(back);
        self.cube.set_face(left);
        self.cube.set_face(right);
        self.cube.set_face(top);
    }

    fn move_double_u(&mut self, reverse: bool) {
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut back = self.cube.get_face(FacePosition::Back);
        let mut right = self.cube.get_face(FacePosition::Right);
        let mut top = self.cube.get_face(FacePosition::Top);
        let saved_top = self.cube.get_face(FacePosition::Top);

        if reverse {
            let save_one = front.get_line(0);
            let save_two = front.get_line(1);
            front.set_line(0, left.get_line(0));
            front.set_line(1, left.get_line(1));
            left.set_line(0, back.get_line(0));
            left.set_line(1, back.get_line(1));
            back.set_line(0, right.get_line(0));
            back.set_line(1, right.get_line(1));
            right.set_line(0, save_one);
            right.set_line(1, save_two);
            top.set_column(0, (saved_top.get_line(0).2, saved_top.get_line(0).1, saved_top.get_line(0).0));
            top.set_column(1, (saved_top.get_line(1).2, saved_top.get_line(1).1, saved_top.get_line(1).0));
            top.set_column(2, (saved_top.get_line(2).2, saved_top.get_line(2).1, saved_top.get_line(2).0));
        } else {
            let save_one = front.get_line(0);
            let save_two = front.get_line(1);
            front.set_line(0, right.get_line(0));
            front.set_line(1, right.get_line(1));
            right.set_line(0, back.get_line(0));
            right.set_line(1, back.get_line(1));
            back.set_line(0, left.get_line(0));
            back.set_line(1, left.get_line(1));
            left.set_line(0, save_one);
            left.set_line(1, save_two);
            top.set_column(0, saved_top.get_line(2));
            top.set_column(1, saved_top.get_line(1));
            top.set_column(2, saved_top.get_line(0));
        }
        self.cube.set_face(front);
        self.cube.set_face(back);
        self.cube.set_face(left);
        self.cube.set_face(right);
        self.cube.set_face(top);
    }

    fn move_d(&mut self, reverse: bool) {
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut back = self.cube.get_face(FacePosition::Back);
        let mut right = self.cube.get_face(FacePosition::Right);
        let mut down = self.cube.get_face(FacePosition::Down);
        let saved_down = self.cube.get_face(FacePosition::Down);

        if reverse {
            let save_state = front.get_line(2);
            front.set_line(2, right.get_line(2));
            right.set_line(2, back.get_line(2));
            back.set_line(2, left.get_line(2));
            left.set_line(2, save_state);
            down.set_line(0, saved_down.get_column(2));
            down.set_line(1, saved_down.get_column(1));
            down.set_line(2, saved_down.get_column(0));
        } else {
            let save_state = front.get_line(2);
            front.set_line(2, left.get_line(2));
            left.set_line(2, back.get_line(2));
            back.set_line(2, right.get_line(2));
            right.set_line(2, save_state);
            down.set_line(0, (saved_down.get_column(0).2, saved_down.get_column(0).1, saved_down.get_column(0).0));
            down.set_line(1, (saved_down.get_column(1).2, saved_down.get_column(1).1, saved_down.get_column(1).0));
            down.set_line(2, (saved_down.get_column(2).2, saved_down.get_column(2).1, saved_down.get_column(2).0));
        }
        self.cube.set_face(front);
        self.cube.set_face(back);
        self.cube.set_face(left);
        self.cube.set_face(right);
        self.cube.set_face(down);
    }

    fn move_double_d(&mut self, reverse: bool) {
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut back = self.cube.get_face(FacePosition::Back);
        let mut right = self.cube.get_face(FacePosition::Right);
        let mut down = self.cube.get_face(FacePosition::Down);
        let saved_down = self.cube.get_face(FacePosition::Down);

        if reverse {
            let save_one = front.get_line(1);
            let save_two = front.get_line(2);
            front.set_line(1, right.get_line(1));
            front.set_line(2, right.get_line(2));
            right.set_line(1, back.get_line(1));
            right.set_line(2, back.get_line(2));
            back.set_line(1, left.get_line(1));
            back.set_line(2, left.get_line(2));
            left.set_line(1, save_one);
            left.set_line(2, save_two);
            down.set_line(0, saved_down.get_column(2));
            down.set_line(1, saved_down.get_column(1));
            down.set_line(2, saved_down.get_column(0));
        } else {
            let save_one = front.get_line(1);
            let save_two = front.get_line(2);
            front.set_line(1, left.get_line(1));
            front.set_line(2, left.get_line(2));
            left.set_line(1, back.get_line(1));
            left.set_line(2, back.get_line(2));
            back.set_line(1, right.get_line(1));
            back.set_line(2, right.get_line(2));
            right.set_line(1, save_one);
            right.set_line(2, save_two);
            down.set_line(0, (saved_down.get_column(0).2, saved_down.get_column(0).1, saved_down.get_column(0).0));
            down.set_line(1, (saved_down.get_column(1).2, saved_down.get_column(1).1, saved_down.get_column(1).0));
            down.set_line(2, (saved_down.get_column(2).2, saved_down.get_column(2).1, saved_down.get_column(2).0));
        }
        self.cube.set_face(front);
        self.cube.set_face(back);
        self.cube.set_face(left);
        self.cube.set_face(right);
        self.cube.set_face(down)
    }

    fn move_l(&mut self, reverse: bool) {
        let mut left = self.cube.get_face(FacePosition::Left);
        let saved_left = self.cube.get_face(FacePosition::Left);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_face = top.get_column(0);

        if reverse {
            top.set_column(0, front.get_column(0));
            front.set_column(0, down.get_column(0));
            down.set_column(0, (back.get_column(2).2, back.get_column(2).1, back.get_column(2).0));
            back.set_column(2, (saved_face.2, saved_face.1, saved_face.0));
            left.set_column(0, (saved_left.get_line(0).2, saved_left.get_line(0).1, saved_left.get_line(0).0));
            left.set_column(1, (saved_left.get_line(1).2, saved_left.get_line(1).1, saved_left.get_line(1).0));
            left.set_column(2, (saved_left.get_line(2).2, saved_left.get_line(2).1, saved_left.get_line(2).0));
        } else {
            top.set_column(0, (back.get_column(2).2, back.get_column(2).1, back.get_column(2).0));
            back.set_column(2, down.get_column(0));
            down.set_column(0, front.get_column(0));
            front.set_column(0, saved_face);
            left.set_column(0, saved_left.get_line(2));
            left.set_column(1, saved_left.get_line(1));
            left.set_column(2, saved_left.get_line(0));
        }
        self.cube.set_face(top);
        self.cube.set_face(front);
        self.cube.set_face(down);
        self.cube.set_face(back);
        self.cube.set_face(left);
    }

    fn move_double_l(&mut self, reverse: bool) {
        let mut left = self.cube.get_face(FacePosition::Left);
        let saved_left = self.cube.get_face(FacePosition::Left);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_face_one = top.get_column(0);
        let saved_face_two = top.get_column(1);

        if reverse {
            top.set_column(0, front.get_column(0));
            top.set_column(1, front.get_column(1));
            front.set_column(0, down.get_column(0));
            front.set_column(1, down.get_column(1));
            down.set_column(0, (back.get_column(2).2, back.get_column(2).1, back.get_column(2).0));
            down.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(2, (saved_face_one.2, saved_face_one.1, saved_face_one.0));
            back.set_column(1, (saved_face_two.2, saved_face_two.1, saved_face_two.0));
            left.set_column(0, (saved_left.get_line(0).2, saved_left.get_line(0).1, saved_left.get_line(0).0));
            left.set_column(1, (saved_left.get_line(1).2, saved_left.get_line(1).1, saved_left.get_line(1).0));
            left.set_column(2, (saved_left.get_line(2).2, saved_left.get_line(2).1, saved_left.get_line(2).0));
        } else {
            top.set_column(0, (back.get_column(2).2, back.get_column(2).1, back.get_column(2).0));
            top.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(2, down.get_column(0));
            back.set_column(1, down.get_column(1));
            down.set_column(0, front.get_column(0));
            down.set_column(1, front.get_column(1));
            front.set_column(0, saved_face_one);
            front.set_column(1, saved_face_two);
            left.set_column(0, saved_left.get_line(2));
            left.set_column(1, saved_left.get_line(1));
            left.set_column(2, saved_left.get_line(0));
        }
        self.cube.set_face(top);
        self.cube.set_face(front);
        self.cube.set_face(down);
        self.cube.set_face(back);
        self.cube.set_face(left);
    }

    fn move_r(&mut self, reverse: bool) {
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_right = self.cube.get_face(FacePosition::Right);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_face = top.get_column(2);

        if reverse {
            top.set_column(2, (back.get_column(0).2, back.get_column(0).1, back.get_column(0).0));
            back.set_column(0, down.get_column(2));
            down.set_column(2, front.get_column(2));
            front.set_column(2, saved_face);
            right.set_column(0, (saved_right.get_line(0).2, saved_right.get_line(0).1, saved_right.get_line(0).0));
            right.set_column(1, (saved_right.get_line(1).2, saved_right.get_line(1).1, saved_right.get_line(1).0));
            right.set_column(2, (saved_right.get_line(2).2, saved_right.get_line(2).1, saved_right.get_line(2).0));
        } else {
            top.set_column(2, front.get_column(2));
            front.set_column(2, down.get_column(2));
            down.set_column(2, (back.get_column(0).2, back.get_column(0).1, back.get_column(0).0));
            back.set_column(0, (saved_face.2, saved_face.1, saved_face.0));
            right.set_column(0, saved_right.get_line(2));
            right.set_column(1, saved_right.get_line(1));
            right.set_column(2, saved_right.get_line(0));
        }
        self.cube.set_face(top);
        self.cube.set_face(front);
        self.cube.set_face(down);
        self.cube.set_face(back);
        self.cube.set_face(right);
    }

    fn move_double_r(&mut self, reverse: bool) {
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_right = self.cube.get_face(FacePosition::Right);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_face_one = top.get_column(2);
        let saved_face_two = top.get_column(1);

        if reverse {
            top.set_column(2, (back.get_column(0).2, back.get_column(0).1, back.get_column(0).0));
            top.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(0, down.get_column(2));
            back.set_column(1, down.get_column(1));
            down.set_column(2, front.get_column(2));
            down.set_column(1, front.get_column(1));
            front.set_column(2, saved_face_one);
            front.set_column(1, saved_face_two);
            right.set_column(0, (saved_right.get_line(0).2, saved_right.get_line(0).1, saved_right.get_line(0).0));
            right.set_column(1, (saved_right.get_line(1).2, saved_right.get_line(1).1, saved_right.get_line(1).0));
            right.set_column(2, (saved_right.get_line(2).2, saved_right.get_line(2).1, saved_right.get_line(2).0));
        } else {
            top.set_column(2, front.get_column(2));
            top.set_column(1, front.get_column(1));
            front.set_column(2, down.get_column(2));
            front.set_column(1, down.get_column(1));
            down.set_column(2, (back.get_column(0).2, back.get_column(0).1, back.get_column(0).0));
            down.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(0, (saved_face_one.2, saved_face_one.1, saved_face_one.0));
            back.set_column(1, (saved_face_two.2, saved_face_two.1, saved_face_two.0));
            right.set_column(0, saved_right.get_line(2));
            right.set_column(1, saved_right.get_line(1));
            right.set_column(2, saved_right.get_line(0));
        }
        self.cube.set_face(top);
        self.cube.set_face(front);
        self.cube.set_face(down);
        self.cube.set_face(back);
        self.cube.set_face(right);
    }

    fn move_m(&mut self, reverse: bool) {
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut front = self.cube.get_face(FacePosition::Front);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_face = top.get_column(1);

        if reverse {
            top.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(1, down.get_column(1));
            down.set_column(1, front.get_column(1));
            front.set_column(1, saved_face);
        } else {
            top.set_column(1, front.get_column(1));
            front.set_column(1, down.get_column(1));
            down.set_column(1, (back.get_column(1).2, back.get_column(1).1, back.get_column(1).0));
            back.set_column(1, (saved_face.2, saved_face.1, saved_face.0));
        }
        self.cube.set_face(top);
        self.cube.set_face(front);
        self.cube.set_face(down);
        self.cube.set_face(back);
    }

    fn move_f(&mut self, reverse: bool) {
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_face = top.get_line(2);
        let mut front = self.cube.get_face(FacePosition::Front);
        let saved_front = self.cube.get_face(FacePosition::Front);

        if reverse {
            top.set_line(2, right.get_column(0));
            right.set_column(0, (down.get_line(0).2, down.get_line(0).1, down.get_line(0).0));
            down.set_line(0, left.get_column(2));
            left.set_column(2, (saved_face.2, saved_face.1, saved_face.0));
            front.set_line(0, saved_front.get_column(2));
            front.set_line(1, saved_front.get_column(1));
            front.set_line(2, saved_front.get_column(0));
        } else {
            top.set_line(2, (left.get_column(2).2, left.get_column(2).1, left.get_column(2).0));
            left.set_column(2, down.get_line(0));
            down.set_line(0, (right.get_column(0).2, right.get_column(0).1, right.get_column(0).0));
            right.set_column(0, saved_face);
            front.set_line(0, (saved_front.get_column(0).2, saved_front.get_column(0).1, saved_front.get_column(0).0));
            front.set_line(1, (saved_front.get_column(1).2, saved_front.get_column(1).1, saved_front.get_column(1).0));
            front.set_line(2, (saved_front.get_column(2).2, saved_front.get_column(2).1, saved_front.get_column(2).0));
        }
        self.cube.set_face(top);
        self.cube.set_face(left);
        self.cube.set_face(down);
        self.cube.set_face(right);
        self.cube.set_face(front);
    }

    fn move_b(&mut self, reverse: bool) {
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_face = top.get_line(0);
        let mut back = self.cube.get_face(FacePosition::Back);
        let saved_back = self.cube.get_face(FacePosition::Back);

        if reverse {
            top.set_line(0, (left.get_column(0).2, left.get_column(0).1, left.get_column(0).0));
            left.set_column(0, down.get_line(2));
            down.set_line(2, (right.get_column(2).2, right.get_column(2).1, right.get_column(2).0));
            right.set_column(2, saved_face);
            back.set_line(0, saved_back.get_column(2));
            back.set_line(1, saved_back.get_column(1));
            back.set_line(2, saved_back.get_column(0));
        } else {
            top.set_line(0, right.get_column(2));
            right.set_column(2, (down.get_line(2).2, down.get_line(2).1, down.get_line(2).0));
            down.set_line(2, left.get_column(0));
            left.set_column(0, (saved_face.2, saved_face.1, saved_face.0));
            back.set_column(0, saved_back.get_line(2));
            back.set_column(1, saved_back.get_line(1));
            back.set_column(2, saved_back.get_line(0));
        }
        self.cube.set_face(top);
        self.cube.set_face(left);
        self.cube.set_face(down);
        self.cube.set_face(right);
        self.cube.set_face(back);
    }

    fn translate_r_to_f(&mut self, _: bool) {
        self.move_double_u(false);
        self.move_d(true);
    }

    fn translate_l_to_f(&mut self, _: bool) {
        self.move_double_u(true);
        self.move_d(false);
    }

    fn translate_u_to_f(&mut self, _: bool) {
        self.move_double_l(false);
        self.move_r(true);
    }

    fn translate_d_to_f(&mut self, _: bool) {
        self.move_double_r(false);
        self.move_l(true);
    }

    fn translate_r_to_u(&mut self, _: bool) {
        self.move_f(true);
        self.move_b(false);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_face = top.get_line(1);

        top.set_line(1, right.get_column(1));
        right.set_column(1, (down.get_line(1).2, down.get_line(1).1, down.get_line(1).0));
        down.set_line(1, left.get_column(1));
        left.set_column(1, (saved_face.2, saved_face.1, saved_face.0));
        self.cube.set_face(top);
        self.cube.set_face(right);
        self.cube.set_face(down);
        self.cube.set_face(left);
    }

    fn translate_l_to_u(&mut self, _: bool) {
        self.move_f(false);
        self.move_b(true);
        let mut top = self.cube.get_face(FacePosition::Top);
        let mut left = self.cube.get_face(FacePosition::Left);
        let mut down = self.cube.get_face(FacePosition::Down);
        let mut right = self.cube.get_face(FacePosition::Right);
        let saved_face = top.get_line(1);

        top.set_line(1, (left.get_column(1).2, left.get_column(1).1, left.get_column(1).0));
        left.set_column(1, down.get_line(1));
        down.set_line(1, (right.get_column(1).2, right.get_column(1).1, right.get_column(1).0));
        right.set_column(1, saved_face);
        self.cube.set_face(top);
        self.cube.set_face(right);
        self.cube.set_face(down);
        self.cube.set_face(left);
    }
}
