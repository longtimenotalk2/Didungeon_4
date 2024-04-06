use crate::game::common::*;

pub struct Unit {
  // 基础
  pub id : Id,
  pub name : String,
  pub team : Team,

  // 基础属性
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

  // 简单索引
  pub fn is_bound(&self) -> bool {
    self.bound > 0
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
}