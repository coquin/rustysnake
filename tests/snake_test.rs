use rustysnake::snake::Snake;

#[test]
fn test_new() {
    let s = Snake::new();
    assert_eq!(s.len(), 100);
}
