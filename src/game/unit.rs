mod data_ally;
mod data_enemy;
mod show_unit;

use crate::game::common::*;
use super::skill::Skill;
use super::buff::Buff;
use colorful::Color;
use colorful::Colorful;
use std::collections::HashMap;

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
  pub atk_shoot : i32,
  pub def_shoot : i32,
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
  buffs : HashMap<Buff, i32>,
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
    atk_shoot : i32,
    def_shoot : i32,
    agi : i32,
    dex : i32,
    luck : i32,
    tie : i32,
    struggle : i32,
    rescue : i32,
    mut addition_skills : Vec<Skill>,
  ) -> Self {
    let mut skills = Skill::basic();
    skills.append(&mut addition_skills);
    Self {
      id,
      name,
      team,
      skills,
      hp_max,
      sp_max,
      tp_max,
      atk_melee,
      def_melee,
      atk_shoot,
      def_shoot,
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
      buffs : HashMap::new(),
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

  pub fn is_bound_full(&self) -> bool {
    self.bound >= 5
  }

  pub fn sp(&self) -> i32 {
    self.sp
  }

  pub fn sp_add(&mut self, mount : i32) {
    self.sp += mount;
    if self.sp > self.sp_max {
      self.sp = self.sp_max;
    }
  }

  pub fn sp_sub(&mut self, mount : i32) {
    self.sp -= mount;
    if self.sp < 0 {self.sp = 0;}
  }

  pub fn tp(&self) -> i32 {
    self.tp
  }

  pub fn tp_add(&mut self, mount : i32) {
    self.tp += mount;
    if self.tp > self.tp_max {self.tp = self.tp_max;}
  }

  pub fn tp_sub(&mut self, mount : i32) {
    self.tp -= mount;
    if self.tp < 0 {self.tp = 0;}
  }

  pub fn buffs(&self) -> &HashMap<Buff, i32> {
    &self.buffs
  }

  pub fn with_buff(&self, buff : Buff) -> bool {
    self.buffs.contains_key(&buff)
  }

  pub fn refresh_buff(&mut self, buff : Buff, stack : i32) {
    self.buffs.entry(buff).and_modify(|s| *s = stack).or_insert(stack);
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

  pub fn atk_shoot(&self) -> f64 {
    let mut atk = self.atk_shoot as f64;
    if self.is_weak() {
      atk *= 0.8;
    }
    atk
  }

  pub fn def_shoot(&self) -> f64 {
    self.def_shoot as f64
  }
  
  pub fn agi(&self) -> f64 {
    let mut spd = self.agi as f64;
    if self.is_bound() {
      spd /= 1.5;
      if self.is_bound_full() {
        spd /= 2.0;
      }
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

  pub fn can_be_subdue(&self) -> bool {
    self.is_weak() || self.with_buff(Buff::Surrender)
  }

  pub fn is_unhealth(&self) -> bool {
    self.hp as f64 / self.hp_max as f64 <= 0.5
  }

  pub fn can_block(&self) -> bool {
    !self.is_bound()
  }

  pub fn bound_add(&mut self, n : i32) {
    self.bound += n;
    if self.bound > 5 {
      self.bound = 5;
    }
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
    let buffs_now = self.buffs.clone();
    for buff in buffs_now.keys() {
      self.buffs.entry(*buff).and_modify(|s| *s -= 1);
      if let Some(v) = self.buffs.get(buff) {
        if *v <= 0 {
          self.buffs.remove(buff);
        }
      }
    }
    if self.is_bound() {
      self.hp += self.hp_max / 20;
      if self.hp > self.hp_max {
        self.hp = self.hp_max;
      }
      // if self.is_weak() {
      //   self.hp += self.hp_max / 20;
      // } 
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
