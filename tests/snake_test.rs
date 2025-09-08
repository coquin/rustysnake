use rustysnake::snake::Snake;

#[test]
fn test_eq() {
    assert!(Snake::new(3) == Snake::new(3));
    assert!(Snake::new(3) != Snake::new(4));
}
