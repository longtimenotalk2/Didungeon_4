use super::super::unit::Unit;
use crate::game::common::*;
use crate::game::skill::Skill;

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
      100, // atk_shoot
      100, // def_shoot
      200, // agi
      100, // dex
      100, // luck
      3, // tie
      1, // struggle
      2, // rescue
      vec!(), // addition skills
    );
    u.bound_add(4);
    u.take_dmg(399);
    u
  }
  
  pub fn new_test_b(id : Id) -> Self {
    let u = Self::new(
      id,
      "射  手".to_string(),
      Team::Ally,
      400, // hp_max
      160, // sp_max
      200, // tp_max
      100, // atk_melee
      100, // def_melee
      100, // atk_shoot
      100, // def_shoot
      200, // agi
      100, // dex
      100, // luck
      3, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::Shoot), // addition skills
    );
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
      111, // atk_shoot
      123, // def_shoot
      100, // agi
      117, // dex
      134, // luck
      4, // tie
      2, // struggle
      3, // rescue
      vec!(Skill::Shoot), // addition skills
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
      137, // def_melee
      83, // atk_shoot
      105, // def_shoot
      103, // agi
      115, // dex
      96, // luck
      2, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::Xuliyiji, Skill::Cangyanzhihun), // addition skills
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
      118, // atk_shoot
      120, // def_shoot
      124, // agi
      117, // dex
      134, // luck
      3, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::RoundhouseKick) // addition skills
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
      126, // atk_shoot
      117, // def_shoot
      112, // agi
      135, // dex
      102, // luck
      3, // tie
      1, // struggle
      2, // rescue
      vec!(Skill::Shoot), // addition skills
    )
  }

    
}