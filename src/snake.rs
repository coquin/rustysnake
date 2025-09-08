#[derive(PartialEq)]
pub struct Snake {
    length: u32,
}

impl Snake {
    pub fn new(len: u32) -> Snake {
        Snake { length: len }
    }

    pub fn len(&self) -> u32 {
        self.length
    }
}
