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

    pub fn dots(&self) -> Vec<u8> {
        vec![self.a, self.b]
    }

    pub fn into_played_stone_by_right(self, right: u8) -> PlayedStone {
        let left = if self.b == right { self.a } else { self.b };
        PlayedStone { left, right }
    }

    pub fn into_played_stone_by_left(self, left: u8) -> PlayedStone {
        let right = if self.b == left { self.a } else { self.b };
        PlayedStone { left, right }
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayedStone {
    left: u8,
    right: u8,
}

impl Display for PlayedStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.left, self.right)
    }
}

impl PlayedStone {
    pub fn left(&self) -> u8 {
        self.left
    }

    pub fn right(&self) -> u8 {
        self.right
    }
}
