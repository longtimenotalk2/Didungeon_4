

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

impl Dir {
  pub fn anti(self) -> Dir {
    match self {
      Dir::Left => Dir::Right,
      Dir::Right => Dir::Left,
      Dir::None => Dir::None,
    }
  }
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
  Border(Dir),
}

impl Target {
  pub fn to_id(&self) -> Option<Id> {
    match self {
      Self::Single(id) => Some(*id),
      _ => None,
    }
  }
}
