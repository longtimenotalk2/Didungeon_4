use std::io;
use super::super::board::Board;
use crate::game::common::*;
use crate::game::skill::Skill;
use colorful::Color;
use colorful::Colorful;
use rand::prelude::*;

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
      if ops == "\n" {
        ops = "0".to_string();
      }
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
          txt += &format!("{i} : {}{}", tar.colored_name(), tar.hp_bar());
          if skill == &Skill::Melee {
            let be = self.melee_expect(*id, *idt);
            if be.is_back {
              txt += &"背刺".color(Color::Red).bold().to_string();
            }
            let hit = be.hit;
            let cri = be.cri;
            let dmg = be.dmg;
            txt += &format!("命{}%,伤{}~{},暴{}%", hit.to_string(), dmg, 2*dmg, cri);
          }
        },
        Target::Border(dir) => {
          match dir {
            Dir::Left => txt += &format!("{i} : {}", "上边界".color(Color::BlueViolet)),
            Dir::Right => txt += &format!("{i} : {}", "下边界".color(Color::BlueViolet)),
            _ => unreachable!(),
          }
        }
      }
      txt += "\n";
    }
    txt += &format!("{len} : 返回上一级");
    println!("{}", txt);
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if ops == "\n" {
        ops = "0".to_string();
      }
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

  pub fn choose_skill_ai(&self, id : Id, rng : &mut ThreadRng) -> Skill {
    let unit = self.id2unit(id);
    let mut tiers = vec!(Vec::new(), Vec::new(),Vec::new());

    
    for skill in unit.skills() {
      if unit.can_skill(skill) {
        if skill.is_no_target() || skill.find_target(self, id).len() > 0 {
          let skill = skill.clone();
          match skill {
            Skill::Subdue => tiers[0].push(skill),
            Skill::Wait => tiers[2].push(skill),
            Skill::Dash => {},
            _ => tiers[1].push(skill),
          }
        }
      }
    }
    for tier in tiers {
      if tier.len() > 0 {
        return tier.choose(rng).unwrap().clone();
      }
    }
    unreachable!()
  }

  pub fn choose_target_ai(&self, targets : &[Target], rng : &mut ThreadRng) -> Target {
    targets.choose(rng).unwrap().clone()
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