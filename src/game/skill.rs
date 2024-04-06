pub mod battle;

use crate::game::common::*;
use super::unit::Unit;


pub enum Skill {
  Melee,
  Subdue,
  Struggle,
  Rescue,
  Wait,
}

impl Skill {
  pub fn basic() -> Vec<Self> {
    vec![Self::Melee, Self::Subdue, Self::Struggle, Self::Rescue, Self::Wait]
  }
}

impl Unit {
  pub fn can_skill(&self, skill : &Skill) -> bool {
    match skill {
      Skill::Melee => !self.is_bound(),
      Skill::Subdue => !self.is_bound(),
      Skill::Struggle => self.is_bound(),
      Skill::Rescue => !self.is_bound(),
      Skill::Wait => true,
    }
  }
}