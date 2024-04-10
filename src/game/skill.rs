pub mod battle;
mod movement;
mod scan;
mod rope;

use crate::game::common::*;
use super::unit::Unit;
use colorful::Color;
use colorful::Colorful;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Catagory {
  // basic
  Melee,
  Shoot,
  Special,
  Rope,
  Dash,
  Wait,
  // special
  
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

  // YeLin
  Xuliyiji,
  Cangyanzhihun,

  // Alyssa
  RoundhouseKick,
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
      Self::Xuliyiji => "蓄力一击".to_string(),
      Self::Cangyanzhihun => "苍炎之魂".to_string(),
      Self::RoundhouseKick => "回旋踢".to_string(),
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

  pub fn cost(&self) -> (i32, i32) {
    match self {
      Skill::Shoot => (5, 0),
      Skill::Xuliyiji => (10, 0),
      Skill::Cangyanzhihun => (0, 25),
      Skill::RoundhouseKick => (0, 30),
      _ => (0, 0)
    }
  }

  pub fn cost_exe(&self, unit : &mut Unit) {
    let (sp, tp) = self.cost();
    unit.sp_sub(sp);
    unit.tp_sub(tp);
  }
}

impl Unit {
  pub fn can_skill_or_reason(&self, skill : &Skill) -> Result<String, String> {
    let emp = String::new();
    let cost_judge = |cost : (i32, i32)| -> Result<String, String> {
      let (sp, tp) = cost;
      let mut ok = true;
      let mut txt = String::new();
      if sp > 0 {
        txt += &format!("sp: {sp}").color(Color::Blue).to_string();
        if sp > self.sp() {
          txt += &"<不足>".color(Color::Red).to_string();
          ok = false;
        }
      }
      if tp > 0 {
        txt += &format!("tp: {tp}").color(Color::Green).to_string();
        if tp > self.tp() {
          txt += &"<不足>".color(Color::Red).to_string();
          ok = false;
        }
      }
      if ok {Ok(txt)} else {Err(txt)}
    };
    let bound_then_cant = || -> Result<String, String> {
      if self.is_bound() {Err(format!("束缚中，无法发动{}", Skill::Melee.to_string()))} else {cost_judge(skill.cost())}
    };
    match skill {
      Skill::Struggle => {
        if self.is_bound() {cost_judge(skill.cost())} else {Err(format!("未被束缚，无法发动{}", Skill::Struggle.to_string()))}
      },
      Skill::Wait => Ok(emp),
      _ => bound_then_cant(),
    }
  }

  pub fn can_skill(&self, skill : &Skill) -> bool {
    self.can_skill_or_reason(skill).is_ok()
  }
}