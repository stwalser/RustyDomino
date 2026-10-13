use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Stone {
    a: u8,
    b: u8,
}

impl Stone {
    pub fn new(a: u8, b: u8) -> Self {
        Self { a, b }
    }

    pub fn is_double(&self) -> bool {
        self.a == self.b
    }

    pub fn dot_sum(&self) -> u8 {
        self.a + self.b
    }

    pub fn double_value(&self) -> Option<u8> {
        if self.is_double() { Some(self.a) } else { None }
    }

    pub fn get_dots(&self) -> Vec<u8> {
        vec![self.a, self.b]
    }

    pub fn into_played_stone(self, closed: u8) -> PlayedStone {
        let open = if self.b == closed { self.a } else { self.b };
        PlayedStone { open, closed }
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayedStone {
    closed: u8,
    open: u8,
}

impl Display for PlayedStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.closed, self.open)
    }
}

impl PlayedStone {
    pub fn get_open(&self) -> u8 {
        self.open
    }
}
