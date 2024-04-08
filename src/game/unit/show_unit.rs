use super::super::unit::Unit;
use crate::game::common::*;
use colorful::Color;
use colorful::Colorful;

impl Unit {
  pub fn show(&self) {
    println!("{}", self.to_string());
  }
  
  pub fn to_string(&self) -> String {
    let mut s = String::new();
    // at
    s += &format!("[{:>3}]", self.at() as i32);
    
    if self.is_bound() {
      s += &format!("束{} ", self.bound());
    }
    s += &self.colored_name();
    s += match self.dir() {
      Dir::Left => " ↑",
      Dir::Right => " ↓",
      Dir::None => " ",
    };
    s += " ";
    s += &if self.is_weak() {
      format!("{:>3}/{:>3}", self.hp.to_string().color(Color::Red), self.hp_max)
    } else {
      format!("{:>3}/{:>3}", self.hp.to_string(), self.hp_max)
    };

    // 属性
    s += "  ";
    s += &format!("攻{:>3} ", self.atk_melee);
    s += &format!("防{:>3} ", self.def_melee);
    s += &format!("速{:>3} ", self.agi);
    s += &format!("技{:>3} ", self.dex);
    s += &format!("运{:>3} ", self.luck);
    
    s
  }
}