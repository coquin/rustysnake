use rustysnake::snake::Direction::{DOWN, LEFT, RIGHT, UP};
use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    let s = Snake::new(UP, 3);
    assert_eq!(s, Snake::new(UP, 3));
    assert_ne!(s, Snake::new(UP, 4));
    assert_ne!(s, Snake::new(DOWN, 3));
    assert_ne!(s, Snake::new(LEFT, 3));
    assert_ne!(s, Snake::new(RIGHT, 3));
}

#[test]
fn test_restore() {
    assert_eq!(Snake::new(UP, 3), Snake::restore(vec![UP, UP, UP]));
    assert_eq!(Snake::new(DOWN, 3), Snake::restore(vec![DOWN, DOWN, DOWN]));
    assert_eq!(Snake::new(LEFT, 3), Snake::restore(vec![LEFT, LEFT, LEFT]));
    assert_eq!(
        Snake::new(RIGHT, 3),
        Snake::restore(vec![RIGHT, RIGHT, RIGHT])
    );
}

// *
// ╔═╗
// ║ ║
// ╚═╝

//          *
// *        ║
// ║   =>   ║
// ║
#[test]
fn test_up_moves_forward() {
    let s = Snake::new(UP, 3);
    let actual = s.forward();
    let expected = Snake::restore(vec![UP, UP, UP]);

    assert_ne!(s, actual);
    assert_eq!(expected, actual);
}

// v           v
// *══   =>   *══
#[test]
fn test_left_moves_forward() {
    let s = Snake::new(LEFT, 3);
    let actual = s.forward();
    let expected = Snake::restore(vec![LEFT, LEFT, LEFT]);

    assert_ne!(s, actual);
    assert_eq!(expected, actual);
}

// *        *╗
// ║   =>    ║
// ║
#[test]
fn test_up_moves_left() {
    let s = Snake::new(UP, 3);
    let actual = s.left();
    let expected = Snake::restore(vec![LEFT, UP, UP]);

    assert_eq!(expected, actual);
}

// *╗         *═╗
//  ║    =>
#[test]
fn test_up_moves_left_and_then_forward() {
    let s = Snake::restore(vec![LEFT, UP, UP]);
    let actual = s.forward();
    let expected = Snake::restore(vec![LEFT, LEFT, UP]);

    assert_eq!(expected, actual);
}

// *═╗   =>   *══
#[test]
fn test_up_moves_left_and_then_forward_and_then_forward() {
    let s = Snake::restore(vec![LEFT, LEFT, UP]);
    let actual = s.forward();
    let expected = Snake::restore(vec![LEFT, LEFT, LEFT]);

    assert_eq!(expected, actual);
}

/*
#[test]
fn test_new() {
    let actual1 = Snake::new(3, UP, P::new(3, 3));
    let expected1 = Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]).unwrap();
    assert_eq!(expected1, actual1);

    let actual2 = Snake::new(3, LEFT, P::new(3, 3));
    let expected2 = Snake::new2(LEFT, vec![P::new(3, 3), P::new(4, 3), P::new(5, 3)]).unwrap();
    assert_eq!(expected2, actual2);

    let actual3 = Snake::new(3, DOWN, P::new(3, 3));
    let expected3 = Snake::new2(DOWN, vec![P::new(3, 3), P::new(3, 2), P::new(3, 1)]).unwrap();
    assert_eq!(expected3, actual3);

    let actual4 = Snake::new(3, RIGHT, P::new(3, 3));
    let expected4 = Snake::new2(RIGHT, vec![P::new(3, 3), P::new(2, 3), P::new(1, 3)]).unwrap();
    assert_eq!(expected4, actual4);
}

#[test]
fn test_new2_should_return_error_when_positions_empty() {
    let r = Snake::new2(LEFT, vec![]);

    assert!(r.is_err());
}

#[test]
fn test_new2_should_allow_creating_snake_of_length_1() {
    let r = Snake::new2(LEFT, vec![P::new(3, 3)]);

    assert!(r.is_ok());
}

#[test]
fn test_forward() {
    let s1 = Snake::new(3, UP, P::new(3, 3));
    assert_eq!(
        s1.forward(),
        Snake::new2(UP, vec![P::new(3, 2), P::new(3, 3), P::new(3, 4)]).unwrap()
    );

    let s2 = Snake::new(3, DOWN, P::new(3, 3));
    assert_eq!(s2.forward(), Snake::new(3, DOWN, P::new(3, 4)));

    let s3 = Snake::new(3, LEFT, P::new(3, 3));
    assert_eq!(s3.forward(), Snake::new(3, LEFT, P::new(2, 3)));

    let s4 = Snake::new(3, RIGHT, P::new(3, 3));
    assert_eq!(s4.forward(), Snake::new(3, RIGHT, P::new(4, 3)));
}

#[test]
fn test_left() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));

    assert_eq!(
        Snake::new(3, Direction::LEFT, Position::new(1, 2)),
        s.left()
    );
    assert_eq!(
        Snake::new(3, Direction::DOWN, Position::new(1, 2)),
        s.left().left()
    );
    assert_eq!(
        Snake::new(3, Direction::RIGHT, Position::new(1, 2)),
        s.left().left().left()
    );
    assert_eq!(
        Snake::new(3, Direction::UP, Position::new(1, 2)),
        s.left().left().left().left()
    );
}

#[test]
fn test_right() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));

    assert_eq!(
        Snake::new(3, Direction::RIGHT, Position::new(1, 2)),
        s.right()
    );
    assert_eq!(
        Snake::new(3, Direction::DOWN, Position::new(1, 2)),
        s.right().right()
    );
    assert_eq!(
        Snake::new(3, Direction::LEFT, Position::new(1, 2)),
        s.right().right().right()
    );
    assert_eq!(
        Snake::new(3, Direction::UP, Position::new(1, 2)),
        s.right().right().right().right()
    );
}
*/
