#[cfg(test)]
mod tests {
    use crate::constants::movements::{D, D_DOUBLE, D_DOUBLE_REVERSE, D_DOUBLE_TWICE, D_REVERSE, D_TWICE, U, U_DOUBLE, U_DOUBLE_REVERSE, U_DOUBLE_TWICE, U_REVERSE, U_TWICE};
    use crate::Cube::Cube::Cube;
    use crate::Face::face::{CaseColor, Face, FacePosition};
    use crate::Movements::Movements::Movements;

    #[test]
    fn init() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let move_cube = Movements::new(&mut cube);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_normal() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_reverse() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U_REVERSE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_twice() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U_TWICE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_double() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U_DOUBLE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_double_reverse() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U_DOUBLE_REVERSE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_u_double_twice() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(U_DOUBLE_TWICE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
    }

    #[test]
    fn move_d_normal() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
    }

    #[test]
    fn move_d_reverse() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D_REVERSE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
    }

    #[test]
    fn move_d_twice() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D_TWICE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
    }

    #[test]
    fn move_d_double() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D_DOUBLE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
    }

    #[test]
    fn move_d_double_reverse() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D_DOUBLE_REVERSE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
    }

    #[test]
    fn move_d_double_twice() {
        let mut cube = Cube::new(vec![
            Face::new(CaseColor::White, FacePosition::Top),
            Face::new(CaseColor::Yellow, FacePosition::Down),
            Face::new(CaseColor::Red, FacePosition::Left),
            Face::new(CaseColor::Blue, FacePosition::Front),
            Face::new(CaseColor::Green, FacePosition::Back),
            Face::new(CaseColor::Orange, FacePosition::Right)]);
        let mut move_cube = Movements::new(&mut cube);

        move_cube.call_movement(D_DOUBLE_TWICE);
        let face = cube.get_face(FacePosition::Front);
        assert_eq!(cube.get_face(FacePosition::Front).get_line(0), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(1), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Front).get_line(2), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(0), (CaseColor::Green, CaseColor::Green, CaseColor::Green));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(1), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Back).get_line(2), (CaseColor::Blue, CaseColor::Blue, CaseColor::Blue));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(0), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(1), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Top).get_line(2), (CaseColor::White, CaseColor::White, CaseColor::White));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(0), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(1), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Down).get_line(2), (CaseColor::Yellow, CaseColor::Yellow, CaseColor::Yellow));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(0), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(1), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Left).get_line(2), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(0), (CaseColor::Orange, CaseColor::Orange, CaseColor::Orange));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(1), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
        assert_eq!(cube.get_face(FacePosition::Right).get_line(2), (CaseColor::Red, CaseColor::Red, CaseColor::Red));
    }
}
