

pub type Id = u32;
pub type Pos = i32;

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

#[derive(Debug, Clone)]
pub enum Target {
  Single(Id),
}

impl Target {
  pub fn to_id(&self) -> Option<Id> {
    match self {
      Self::Single(id) => Some(*id),
      _ => None,
    }
  }
}
