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
    positions: Vec<Position>,
}

impl Snake {
    pub fn new(len: u32, dir: Direction, pos: Position) -> Snake {
        let mut positions = Vec::new();
        let mut cur_pos = pos.clone();

        for i in 1..=len {
            positions.push(cur_pos.clone());
            cur_pos = match dir {
                Direction::UP => cur_pos.down(),
                Direction::LEFT => cur_pos.right(),
                Direction::DOWN => cur_pos.up(),
                Direction::RIGHT => cur_pos.left(),
            }
        }
        Snake {
            length: len,
            direction: dir,
            position: pos,
            positions: positions,
        }
    }

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

    pub fn len(&self) -> u32 {
        self.length
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
