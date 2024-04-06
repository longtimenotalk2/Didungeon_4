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
    s += &self.colored_name();
    s += match self.dir() {
      Dir::Left => " ↑",
      Dir::Right => " ↓",
      Dir::None => " ",
    };
    s += " ";
    s += &if self.is_bound() {
      format!("束{}", self.bound())
    } else {"   ".to_string()};
    s += " ";
    s += &if self.is_weak() {
      format!("{:>3}/{:>3}", self.hp.to_string().color(Color::Red), self.hp_max)
    } else {
      format!("{:>3}/{:>3}", self.hp.to_string(), self.hp_max)
    };
    s
  }
}