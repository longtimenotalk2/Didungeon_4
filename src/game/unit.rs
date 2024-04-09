mod data_ally;
mod data_enemy;
mod show_unit;

use crate::game::common::*;
use super::skill::Skill;
use colorful::Color;
use colorful::Colorful;

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
    let mut cstr = self.name.clone().color(
      match self.team {
        Team::Ally => Color::Blue,
        Team::Enemy => Color::Red,
      }
    );
    if self.is_bound() {
      cstr = cstr.dim();
    }
      
    cstr.to_string()
  }

  pub fn hp_bar(&self) -> String {
    hp_bar(self.hp, self.hp_max)
  }
  
  pub fn is_bound(&self) -> bool {
    self.bound > 0
  }

  // 复杂逻辑
  pub fn atk_melee(&self) -> f64 {
    let mut atk = self.atk_melee as f64;
    if self.is_weak() {
      atk *= 0.8;
    }
    atk
  }

  pub fn def_melee(&self) -> f64 {
    self.def_melee as f64
  }
  
  pub fn agi(&self) -> f64 {
    let mut spd = self.agi as f64;
    if self.is_bound() {
      spd /= 1.5;
    }
    spd
  }

  pub fn dex(&self) -> f64 {
    let mut dex = self.dex as f64;
    if self.is_weak() {
      dex *= 0.8;
    }
    dex
  }

  pub fn luck(&self) -> f64 {
    self.luck as f64
  }

  

  pub fn calc_t(&self) -> f64 {
    self.at / self.agi()
  }

  pub fn t_pass(&mut self, t : f64, to_zero : bool) {
    if to_zero {
      self.at = 0.0;
    } else {
      self.at -= t * self.agi();
    }
  }

  pub fn take_dmg(&mut self, dmg : i32) {
    self.hp -= dmg;
    if self.hp < 0 {
      self.hp = 0;
    }
  }

  pub fn is_weak(&self) -> bool {
    self.hp as f64 / self.hp_max as f64 <= 0.2
  }

  pub fn is_unhealth(&self) -> bool {
    self.hp as f64 / self.hp_max as f64 <= 0.5
  }

  pub fn can_block(&self) -> bool {
    !self.is_bound()
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

  pub fn turn_start(&mut self) {
    self.dir = Dir::None;
    if self.is_bound() {
      if self.is_weak() {
        self.hp += self.hp_max / 20;
      } 
    }
  }

  pub fn zoc(&self) -> Vec<Dir> {
    if !self.is_bound() {
      match self.dir {
        Dir::None => vec!(Dir::Left, Dir::Right),
        Dir::Left => vec!(Dir::Left),
        Dir::Right => vec!(Dir::Right),
      }
    } else {
      vec!()
    }
  }
}

fn hp_bar(hp : i32, hp_max : i32) -> String {
  fn block(i : i32) -> &'static str {
    match i {
      ..=0 => " ",
      1 => "▏",
      2 => "▎",
      3 => "▍",
      4 => "▌",
      5 => "▋",
      6 => "▊",
      7 => "▉",
      8.. => "█",
      _ => unreachable!(),
    }
  }

  let rate = hp as f64 / hp_max as f64;
  let n = 4;
  let color = if rate <= 0.2 {
    Color::Red
  } else if rate <= 0.5 {
    Color::Yellow
  } else {
    Color::Green
  };
  let mut txt = String::new();
  txt += "▕";
  let q = (n * 8) as f64;
  for i in 0..n {
    txt += &block((rate * q - i as f64 * 8.) as i32).color(color).to_string()
  }
  txt += "▏";
  txt
}
