#[derive(PartialEq, Clone)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(PartialEq, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(PartialEq)]
pub struct Snake {
    length: u32,
    direction: Direction,
    position: Position,
}

impl Snake {
    pub fn new(len: u32, dir: Direction, pos: Position) -> Snake {
        Snake {
            length: len,
            direction: dir,
            position: pos,
        }
    }

    pub fn forward(&self) -> Snake {
        Self::new(self.length, self.direction.clone(), self.position.up())
    }

    pub fn left(&self) -> Snake {
        Self::new(self.length, self.direction.left(), self.position.clone())
    }

    pub fn right(&self) -> Snake {
        Self::new(self.length, self.direction.right(), self.position.clone())
    }
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::DOWN,
            Direction::DOWN => Direction::RIGHT,
            Direction::RIGHT => Direction::UP,
            Direction::UP => Direction::LEFT,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
        }
    }
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }

    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}
