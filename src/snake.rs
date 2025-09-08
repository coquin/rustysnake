pub struct Snake {
    length: u64,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { length: 42 }
    }

    pub fn len(&self) -> u64 {
        self.length
    }
}
