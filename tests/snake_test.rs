use rustysnake::snake::Direction;
use rustysnake::snake::Position;
use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));
    assert!(s == Snake::new(3, Direction::UP, Position::new(1, 2)));
    assert!(s != Snake::new(4, Direction::UP, Position::new(1, 2)));
    assert!(s != Snake::new(3, Direction::DOWN, Position::new(1, 2)));
    assert!(s != Snake::new(3, Direction::UP, Position::new(1, 1)));
}

#[test]
fn test_forward() {
    let s1 = Snake::new(3, Direction::UP, Position::new(1, 2));
    let s2 = s1.forward();

    assert!(s1 != s2);

    let s3 = s2.forward();
    assert!(s2 != s3);
}

#[test]
fn test_left() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));

    assert!(Snake::new(3, Direction::LEFT, Position::new(1, 2)) == s.left());
    assert!(Snake::new(3, Direction::DOWN, Position::new(1, 2)) == s.left().left());
    assert!(Snake::new(3, Direction::RIGHT, Position::new(1, 2)) == s.left().left().left());
    assert!(Snake::new(3, Direction::UP, Position::new(1, 2)) == s.left().left().left().left());
}

#[test]
fn test_right() {
    let s = Snake::new(3, Direction::UP, Position::new(1, 2));

    assert!(Snake::new(3, Direction::RIGHT, Position::new(1, 2)) == s.right());
    assert!(Snake::new(3, Direction::DOWN, Position::new(1, 2)) == s.right().right());
    assert!(Snake::new(3, Direction::LEFT, Position::new(1, 2)) == s.right().right().right());
    assert!(Snake::new(3, Direction::UP, Position::new(1, 2)) == s.right().right().right().right());
}
