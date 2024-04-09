pub mod battle;
mod movement;
mod scan;
mod rope;

use crate::game::common::*;
use super::unit::Unit;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Skill {
  Melee,
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
}

impl Unit {
  pub fn can_skill_or_reason(&self, skill : &Skill) -> Result<(), String> {
    match skill {
      Skill::Melee => {
        if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Melee.to_string()))} else {Ok(())}
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