pub mod battle;
mod movement;
mod scan;
mod rope;

use crate::game::common::*;
use super::unit::Unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Catagory {
  Melee,
  Shoot,
  Special,
  Rope,
  Dash,
  Wait,
}

impl Catagory {
  pub fn all() -> Vec<Self> {
    vec![
      Self::Melee,
      Self::Shoot,
      Self::Special,
      Self::Rope,
      Self::Dash,
      Self::Wait,
    ]
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Skill {
  Melee,
  Shoot,
  Subdue,
  Struggle,
  Rescue,
  Dash,
  Wait,
}

impl Skill {
  pub fn basic() -> Vec<Self> {
    vec![Self::Melee, Self::Subdue, Self::Struggle, Self::Rescue, Self::Dash, Self::Wait]
  }

  pub fn to_string(&self) -> String {
    match self {
      Self::Melee => "挥击".to_string(),
      Self::Shoot => "射击".to_string(),
      Self::Subdue => "制服".to_string(),
      Self::Struggle => "挣扎".to_string(),
      Self::Rescue => "拯救".to_string(),
      Self::Dash => "移动".to_string(),
      Self::Wait => "等待".to_string(),
    }
  }

  pub fn is_no_target(&self) -> bool {
    match self {
      Self::Struggle => true,
      Self::Wait => true,
      _ => false,
    }
  }

  pub fn catagory(&self) -> Catagory {
    match self {
      Self::Melee => Catagory::Melee,
      Self::Shoot => Catagory::Shoot,
      Self::Subdue => Catagory::Rope,
      Self::Struggle => Catagory::Rope,
      Self::Rescue => Catagory::Rope,
      Self::Dash => Catagory::Dash,
      Self::Wait => Catagory::Wait,
      _ => Catagory::Special,
    }
  }
}

impl Unit {
  pub fn can_skill_or_reason(&self, skill : &Skill) -> Result<(), String> {
    match skill {
      Skill::Melee => {
        if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Melee.to_string()))} else {Ok(())}
      },
      Skill::Shoot => {
        if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Shoot.to_string()))} else {Ok(())}
      },
      Skill::Subdue => {
        if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Subdue.to_string()))} else {Ok(())}
      },
      Skill::Struggle => {
        if self.is_bound() {Ok(())} else {Err(format!("未被束缚，无法发动{}", Skill::Struggle.to_string()))}
      },
      Skill::Rescue => {
          if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Rescue.to_string()))} else {Ok(())}
      },
      Skill::Dash => {
          if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Rescue.to_string()))} else {Ok(())}
      },
      Skill::Wait => Ok(()),
    }
  }

  pub fn can_skill(&self, skill : &Skill) -> bool {
    self.can_skill_or_reason(skill).is_ok()
  }
}