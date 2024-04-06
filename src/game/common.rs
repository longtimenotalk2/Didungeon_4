

pub type Id = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
  Ally,
  Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
  Left,
  Right,
  None,
}

pub enum ResultBoard {
  Win,
  Lose,
  OutOfTime,
  Panic,
}