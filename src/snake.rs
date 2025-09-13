#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Debug)]
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

    pub fn new2(pos: Position, segments: Vec<Direction>) -> Result<Snake, String> {
        segments
            .first()
            .ok_or("Segments cannot be empty".to_string())
            .map(|s| Snake {
                length: segments.len() as u32,
                direction: s.clone(),
                position: pos,
            })
    }

    pub fn positions(&self) -> Vec<Position> {
        vec![self.position.clone(), self.position.down().clone(), self.position.clone().down().down()]
    }

    pub fn forward(&self) -> Snake {
        let new_position = match self.direction {
            Direction::UP => self.position.up(),
            Direction::DOWN => self.position.down(),
            Direction::LEFT => self.position.left(),
            Direction::RIGHT => self.position.right(),
        };
        Self::new(self.length, self.direction.clone(), new_position)
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
