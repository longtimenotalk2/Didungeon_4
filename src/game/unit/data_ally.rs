use super::super::unit::Unit;
use crate::game::common::*;

impl Unit {
  pub fn new_test_a(id : Id) -> Self {
    let mut u = Self::new(
      id,
      "人  偶".to_string(),
      Team::Ally,
      400, // hp_max
      160, // sp_max
      200, // tp_max
      100, // atk_melee
      100, // def_melee
      100, // agi
      100, // dex
      100, // luck
      3, // tie
      1, // struggle
      2, // rescue
    );
    u.bound_add(4);
    u.take_dmg(399);
    u
  }
  
  pub fn new_noal(id : Id) -> Self {
    Self::new(
      id,
      "诺艾尔".to_string(),
      Team::Ally,
      320, // hp_max
      160, // sp_max
      200, // tp_max
      80, // atk_melee
      90, // def_melee
      100, // agi
      120, // dex
      150, // luck
      4, // tie
      2, // struggle
      3, // rescue
    )
  }

  pub fn new_yelin(id : Id) -> Self {
    Self::new(
      id,
      "叶  琳".to_string(),
      Team::Ally,
      500, // hp_max
      160, // sp_max
      200, // tp_max
      130, // atk_melee
      110, // def_melee
      103, // agi
      115, // dex
      96, // luck
      2, // tie
      1, // struggle
      1, // rescue
    )
  }

  pub fn new_alyssa(id : Id) -> Self {
    Self::new(
      id,
      "艾丽莎".to_string(),
      Team::Ally,
      380, // hp_max
      200, // sp_max
      200, // tp_max
      92, // atk_melee
      95, // def_melee
      124, // agi
      117, // dex
      134, // luck
      3, // tie
      1, // struggle
      2, // rescue
    )
  }

  pub fn new_elis(id : Id) -> Self {
    Self::new(
      id,
      "伊莉丝".to_string(),
      Team::Ally,
      420, // hp_max
      180, // sp_max
      200, // tp_max
      105, // atk_melee
      110, // def_melee
      112, // agi
      135, // dex
      102, // luck
      3, // tie
      1, // struggle
      2, // rescue
    )
  }

    
}