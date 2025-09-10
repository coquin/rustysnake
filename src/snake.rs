#[derive(PartialEq)]
pub struct Snake {
    length: u32,
}

impl Snake {
    pub fn new(len: u32) -> Snake {
        Snake { length: len }
    }

    pub fn forward(&self) -> Snake {
        Self::new(self.length)
    }

    pub fn left(&self) -> Snake {
        Self::new(self.length)
    }

    pub fn right(&self) -> Snake {
        Self::new(self.length)
    }
}
