mod data_ally;
mod data_enemy;
mod show_unit;

use crate::game::common::*;
use super::skill::Skill;

pub struct Unit {
  // 基础
  pub id : Id,
  pub name : String,
  pub team : Team,

  // 技能
  skills : Vec<Skill>,

  // 基础属性
  pub hp_max : i32,
  pub sp_max : i32,
  pub tp_max : i32,
  pub atk_melee : i32,
  pub def_melee : i32,
  pub agi : i32,
  pub dex : i32,
  pub luck : i32,
  pub tie : i32,
  pub struggle : i32,
  pub rescue : i32,

  // 状态
  hp : i32,
  sp : i32,
  tp : i32,
  at : f64,
  bound : i32,
  dir : Dir,
  
}

impl Unit {
  pub fn new(
    id : Id,
    name : String,
    team : Team,
    hp_max : i32,
    sp_max : i32,
    tp_max : i32,
    atk_melee : i32,
    def_melee : i32,
    agi : i32,
    dex : i32,
    luck : i32,
    tie : i32,
    struggle : i32,
    rescue : i32,
  ) -> Self {
    Self {
      id,
      name,
      team,
      skills : Skill::basic(),
      hp_max,
      sp_max,
      tp_max,
      atk_melee,
      def_melee,
      agi,
      dex,
      luck,
      tie,
      struggle,
      rescue,
      hp : hp_max,
      sp : sp_max,
      tp : 10,
      at : 100.0,
      bound : 0,
      dir : Dir::None,
    }
  }

  // 直接索引
  pub fn dir(&self) -> Dir {
    self.dir
  }

  pub fn skills(&self) -> &Vec<Skill> {
    &self.skills
  }
  
  pub fn set_dir(&mut self, dir : Dir) {
    self.dir = dir;
  }

  pub fn at(&self) -> f64 {
    self.at
  }

  pub fn bound(&self) -> i32 {
    self.bound
  }

  // 简单索引
  pub fn colored_name(&self) -> String {
    use colorful::Color;
    use colorful::Colorful;
    self.name.clone().color(
      match self.team {
        Team::Ally => Color::Blue,
        Team::Enemy => Color::Red,
      }
    ).to_string()
  }
  
  pub fn is_bound(&self) -> bool {
    self.bound > 0
  }
  
  pub fn can_block(&self) -> bool {
    !self.is_bound()
  }

  // 复杂逻辑
  fn spd(&self) -> f64 {
    let mut spd = self.agi as f64;
    if self.is_bound() {
      spd /= 2.0;
    }
    spd
  }

  pub fn calc_t(&self) -> f64 {
    self.at / self.spd()
  }

  pub fn t_pass(&mut self, t : f64, to_zero : bool) {
    if to_zero {
      self.at = 0.0;
    } else {
      self.at -= t * self.spd();
    }
  }

  pub fn take_dmg(&mut self, dmg : i32) {
    self.hp -= dmg;
    if self.hp < 0 {
      self.hp = 0;
    }
  }

  pub fn is_weak(&self) -> bool {
    if self.hp <= 0 {return true;}
    self.hp as f64 / self.hp_max as f64 <= 0.15
  }

  pub fn bound_add(&mut self, n : i32) {
    self.bound += n;
  } 

  pub fn bound_sub(&mut self, n : i32) -> bool {
    self.bound -= n;
    if self.bound <= 0 {
      self.bound = 0;
      true
    } else {
      false
    }
  } 

  pub fn at_delay(&mut self, n : f64) {
    self.at += n;
  }
}