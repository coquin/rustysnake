use rustysnake::snake::Direction::{DOWN, LEFT, RIGHT, UP};
use rustysnake::snake::Position as P;
use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    let s = Snake::new(3, UP, P::new(3, 3));
    assert_eq!(s, Snake::new(3, UP, P::new(3, 3)));
    assert_ne!(s, Snake::new(4, UP, P::new(3, 3)));
    assert_ne!(s, Snake::new(3, UP, P::new(2, 3)));
    assert_ne!(s, Snake::new(3, UP, P::new(3, 2)));

    assert_ne!(s, Snake::new(3, DOWN, P::new(3, 3)));
    assert_ne!(s, Snake::new(3, RIGHT, P::new(3, 3)));
    assert_ne!(s, Snake::new(3, LEFT, P::new(3, 3)));

    assert_eq!(
        Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]),
        Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]),
    );

    assert_ne!(
        Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]),
        Snake::new2(DOWN, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]),
    );

    assert_ne!(
        Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4), P::new(3, 5)]),
        Snake::new2(UP, vec![P::new(3, 3), P::new(3, 4)]),
    );
}

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

/*
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
