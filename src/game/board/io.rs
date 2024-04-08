use std::io;
use super::super::board::Board;
use crate::game::common::*;
use crate::game::skill::Skill;
use colorful::Color;
use colorful::Colorful;

impl Board {
  pub fn choose_skill(&self, id : Id) -> Skill {
    let unit = self.id2unit(id);
    let mut txt = String::new();
    let mut valid_i = Vec::new();
    for (i, skill) in unit.skills().iter().enumerate() {
      match unit.can_skill_or_reason(skill) {
        Ok(_) => {
          if skill.is_no_target() || skill.find_target(self, id).len() > 0 {
            valid_i.push(i);
            txt += &format!("{i} : {}", skill.to_string());
          } else {
            txt += &format!("{i} : {}", skill.to_string()).color(Color::DarkGray).to_string();
            txt += &format!(" ({})", "无合法目标".color(Color::DarkGray));
          }
        },
        Err(msg) => {
          txt += &format!("{i} : {}", skill.to_string()).color(Color::DarkGray).to_string();
          txt += &format!(" ({})", msg.color(Color::Red));
        },
      }
      txt += "\n";
    }
    print!("{}", txt);
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if let Ok(op) = ops.trim().parse::<usize>() {
        if valid_i.contains(&op) {
          println!("");
          return unit.skills().get(op).unwrap().clone()
        } else {
          println!("请输入可执行的技能序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }

  pub fn choose_target(&self, id : &Id, skill : &Skill, targets : &[Target]) -> Option<Target> {
    let mut txt = String::new();
    let len = targets.len();
    for (i, target) in targets.iter().enumerate() {
      match target {
        Target::Single(idt) => {
          let tar = self.id2unit(*idt);
          txt += &format!("{i} : {}", tar.colored_name());
          if skill == &Skill::Melee {
            let be = self.melee_expect(*id, *idt);
            let hit = be.hit;
            txt += &format!(" (命中率={}%)", hit.to_string());
          }
        },
      }
      txt += "\n";
    }
    txt += &format!("{len} : 返回上一级");
    println!("{}", txt);
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if let Ok(op) = ops.trim().parse::<usize>() {
        if op < len {
          println!("");
          return Some(targets[op].clone())
        } else if op == len {
          println!("");
          return None;
        } else {
          println!("请输入可执行的技能序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }
}

fn io(title: String, options : &[String], mut default : Option<usize>) -> usize {
  if default.is_none() {
    default = Some(0);
  }
  println!("{}", title);
  // 显示所有选项
  for (i, option) in options.iter().enumerate() {
    println!("{}: {}", i, option);
  }
  // 接受用户输入，如果正确则返回对应的索引
  let len = options.len();
  loop {
    let mut ops = String::new();
    io::stdin().read_line(&mut ops).expect("failed to read line");
    if default.is_some() && &ops == "\n" {
      return default.unwrap()
    }
    if let Ok(op) = ops.trim().parse::<usize>() {
      if op < len {
        return op
      } else {
        println!("输入错误,请输入所给选项前面的数字");
      }
    }else {
      println!("输入错误,请输入一个自然数");
    }
  }
}