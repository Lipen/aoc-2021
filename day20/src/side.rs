#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    Top,
    Bot,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Inner,
    Outer,
}
