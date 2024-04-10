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

    // name
    s += &self.colored_name();
    s += match self.dir() {
      Dir::Left => "↑",
      Dir::Right => "↓",
      Dir::None => " ",
    };
    s += " ";

    // bound
    if self.is_bound() {
      s += &format!("束{}", self.bound());
    } else {
      s += "   ";
    }

    //hp
    s += &self.hp_bar();
    s += &if self.is_weak() {
      format!("{}/{:>3}", format!("{:>3}", self.hp).color(Color::Red), self.hp_max)
    } else {
      format!("{:>3}/{:>3}", self.hp, self.hp_max)
    };

    // sp, at
    s += ", ";
    s += &format!("{:>3}/{:>3}", self.sp, self.sp_max).color(Color::Blue).to_string();
    s += ", ";
    s += &format!("{:>3}/{:>3}", self.tp, self.tp_max).color(Color::Green).to_string();

    // 属性
    fn cc(a : i32, b : i32) -> String {
      if a < b {
        format!("{:>3}", a.to_string()).color(Color::Red).to_string()
      }else if a > b {
        format!("{:>3}", a.to_string()).color(Color::Green).to_string()
      }else {
        format!("{:>3}", a.to_string())
      }
    }
    // s += "  ";
    // s += &format!("攻{} ", cc(self.atk_melee() as i32, self.atk_melee));
    // s += &format!("防{} ", cc(self.def_melee() as i32, self.def_melee));
    // s += &format!("速{} ", cc(self.agi() as i32, self.agi));
    // s += &format!("技{} ", cc(self.dex() as i32, self.dex));
    // s += &format!("运{} ", cc(self.luck() as i32, self.luck));
    
    s
  }
}



