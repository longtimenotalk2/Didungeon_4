use super::super::board::Board;
use crate::game::common::*;
use super::super::unit::Unit;

impl Board {
  pub fn show(&self) {
    println!("{}", self.to_string())
  }

  pub fn to_string(&self) -> String {
    let mut s = String::new();
    for unit in &self.units {
      s += &unit.to_string();
      s += "\n";
    }
    s[..(s.len() - 1)].to_string()
  }
}