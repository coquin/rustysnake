#[derive(PartialEq, Clone)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(PartialEq)]
pub struct Snake {
    length: u32,
    direction: Direction,
}

impl Snake {
    pub fn new(len: u32, dir: Direction) -> Snake {
        Snake {
            length: len,
            direction: dir,
        }
    }

    pub fn forward(&self) -> Snake {
        Self::new(self.length, self.direction.clone())
    }

    pub fn left(&self) -> Snake {
        Self::new(self.length, self.direction.left())
    }

    pub fn right(&self) -> Snake {
        Self::new(self.length, self.direction.right())
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
