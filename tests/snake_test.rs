use rustysnake::snake::Direction;
use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    assert!(Snake::new(3, Direction::UP) == Snake::new(3, Direction::UP));
    assert!(Snake::new(3, Direction::UP) != Snake::new(4, Direction::UP));
}

#[test]
fn test_forward() {
    let s1 = Snake::new(3, Direction::UP);
    let s2 = s1.forward();

    assert!(s1 != s2)
}

#[test]
fn test_left() {
    let s = Snake::new(3, Direction::UP);

    assert!(Snake::new(3, Direction::LEFT) == s.left());
    assert!(Snake::new(3, Direction::DOWN) == s.left().left());
    assert!(Snake::new(3, Direction::RIGHT) == s.left().left().left());
    assert!(Snake::new(3, Direction::UP) == s.left().left().left().left());
}

#[test]
fn test_right() {
    let s = Snake::new(3, Direction::UP);

    assert!(Snake::new(3, Direction::RIGHT) == s.right());
    assert!(Snake::new(3, Direction::DOWN) == s.right().right());
    assert!(Snake::new(3, Direction::LEFT) == s.right().right().right());
    assert!(Snake::new(3, Direction::UP) == s.right().right().right().right());
}
