#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

/*
#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}
*/

#[derive(PartialEq, Debug)]
pub struct Snake {
    segments: Vec<Direction>,
}

impl Snake {
    pub fn new(dir: Direction, len: u32) -> Snake {
        Snake {
            segments: Vec::new(),
        }
    }

    pub fn restore(dirs: Vec<Direction>) -> Snake {
        Snake {
            segments: dirs,
        }
    }

    /*
    pub fn new2(dir: Direction, positions: Vec<Position>) -> Result<Snake, String> {
        positions
            .first()
            .ok_or("Segments cannot be empty".to_string())
            .map(|p| Snake {
                length: positions.len() as u32,
                direction: dir,
                position: p.clone(),
                positions: positions.clone(),
            })
    }
    */

    pub fn forward(&self) -> Snake {
        Snake {
            segments: self.segments.clone(),
        }
    }

    pub fn left(&self) -> Snake {
        Snake {
            segments: self.segments.clone(),
        }
    }

    /*
    pub fn right(&self) -> Snake {
        Self::new(self.length, self.direction.right(), self.position.clone())
    }
    */
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

    /*
    fn right(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
        }
    }*/
}

/*
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
*/
