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
    if self.is_bound() {
      s += &self.colored_name().dim().to_string();
    } else {
      s += &self.colored_name();
    }
    s += match self.dir() {
      Dir::Left => " ↑",
      Dir::Right => " ↓",
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
    s += &hp_bar(self.hp, self.hp_max);
    s += &if self.is_weak() {
      format!("{}/{:>3}", format!("{:>3}", self.hp).color(Color::Red), self.hp_max)
    } else {
      format!("{:>3}/{:>3}", self.hp.to_string(), self.hp_max)
    };

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
    s += "  ";
    s += &format!("攻{} ", cc(self.atk_melee() as i32, self.atk_melee));
    s += &format!("防{} ", cc(self.def_melee() as i32, self.def_melee));
    s += &format!("速{} ", cc(self.agi() as i32, self.agi));
    s += &format!("技{} ", cc(self.dex() as i32, self.dex));
    s += &format!("运{} ", cc(self.luck() as i32, self.luck));
    
    s
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

