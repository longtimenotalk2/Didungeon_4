use super::super::unit::Unit;
use crate::game::common::*;

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
      100, // agi
      100, // dex
      100, // luck
      2, // tie
      1, // struggle
      1, // rescue
    )
  }

  pub fn new_thief(id : Id) -> Self {
    Self::new(
      id,
      "女盗贼".to_string(),
      Team::Enemy,
      400, // hp_max
      160, // sp_max
      200, // tp_max
      85, // atk_melee
      90, // def_melee
      110, // agi
      115, // dex
      105, // luck
      3, // tie
      1, // struggle
      2, // rescue
    )
  }
}