use super::super::unit::Unit;
use crate::game::common::*;
use crate::game::skill::Skill;

impl Unit {
    pub fn new_fighter(id : Id) -> Self {
    Self::new(
      id,
      "女战士".to_string(),
      Team::Enemy,
      500, // hp_max
      160, // sp_max
      200, // tp_max
      100, // atk_melee
      100, // def_melee
      80, // atk_shoot
      90, // def_shoot
      100, // agi
      100, // dex
      80, // luck
      2, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::Whack), // addition skills
    )
  }

  pub fn new_arc(id : Id) -> Self {
    Self::new(
      id,
      "女弓手".to_string(),
      Team::Enemy,
      400, // hp_max
      160, // sp_max
      200, // tp_max
      85, // atk_melee
      90, // def_melee
      95, // atk_shoot
      100, // def_shoot
      110, // agi
      115, // dex
      90, // luck
      3, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::Shoot, Skill::PrecisionSniping, Skill::SecureBound), // addition skills
    )
  }
}