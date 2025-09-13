use rustysnake::snake::Direction;
use rustysnake::snake::Position;
use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));
    assert_eq!(s, Snake::new(3, Direction::UP, Position::new(1, 2)));
    assert_ne!(s, Snake::new(4, Direction::UP, Position::new(1, 2)));
    assert_ne!(s, Snake::new(3, Direction::DOWN, Position::new(1, 2)));
    assert_ne!(s, Snake::new(3, Direction::UP, Position::new(1, 1)));
}

#[test]
fn test_new() {
    let s1 = Snake::new(3, Direction::UP, Position::new(1, 2));
    assert_eq!(s1.positions(), vec![Position::new(1, 2), Position::new(1, 3), Position::new(1, 4)]);

    let s2 = Snake::new(2, Direction::UP, Position::new(1, 2));
    assert_eq!(s1.positions(), vec![Position::new(1, 2), Position::new(1, 3)]);
}

#[test]
fn test_new_snake_segments() {
    let s = Snake::new2(
        Position::new(1, 2),
        vec![Direction::RIGHT, Direction::RIGHT, Direction::RIGHT],
    )
    .unwrap();
    assert_eq!(s, Snake::new(3, Direction::RIGHT, Position::new(1, 2)));
}

#[test]
fn test_should_return_error_when_segments_empty() {
    let r = Snake::new2(Position::new(1, 2), vec![]);

    assert!(r.is_err());
}

#[test]
fn test_forward() {
    let s1 = Snake::new(3, Direction::UP, Position::new(1, 2));
    assert_eq!(
        s1.forward(),
        Snake::new(3, Direction::UP, Position::new(1, 1))
    );

    let s2 = Snake::new(3, Direction::DOWN, Position::new(1, 2));
    assert_eq!(
        s2.forward(),
        Snake::new(3, Direction::DOWN, Position::new(1, 3))
    );

    let s3 = Snake::new(3, Direction::LEFT, Position::new(1, 2));
    assert_eq!(
        s3.forward(),
        Snake::new(3, Direction::LEFT, Position::new(0, 2))
    );

    let s4 = Snake::new(3, Direction::RIGHT, Position::new(1, 2));
    assert_eq!(
        s4.forward(),
        Snake::new(3, Direction::RIGHT, Position::new(2, 2))
    );
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
