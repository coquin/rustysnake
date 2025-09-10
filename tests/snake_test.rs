use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    assert!(Snake::new(3) == Snake::new(3));
    assert!(Snake::new(3) != Snake::new(4));
}

#[test]
fn test_forward() {
    let s1 = Snake::new(3);
    let s2 = s1.forward();

    assert!(s1 != s2)
}

#[test]
fn test_left() {
    let s1 = Snake::new(3);
    let s2 = s1.left();

    assert!(s1 != s2)
}

#[test]
fn test_right() {
    let s1 = Snake::new(3);
    let s2 = s1.right();

    assert!(s1 != s2)
}
