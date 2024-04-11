use std::io;
use super::super::board::Board;
use crate::game::common::*;
use crate::game::skill::{Skill, Catagory};
use colorful::Color;
use colorful::Colorful;
use rand::prelude::*;

impl Board {
  pub fn choose_catagory(&self, id : Id) -> Catagory {
    let unit = self.id2unit(id);
    let mut txt = String::new();
    let mut valid_i = Vec::new();
    let skills = unit.skills();
    for (i, c) in Catagory::all().iter().enumerate() {
      match c {
        Catagory::Melee => {
          match unit.can_skill_or_reason(&Skill::Melee) {
            Ok(_) => {
              if Skill::Melee.find_target(self, id).len() > 0 {
                valid_i.push(i);
                txt += &format!("{i} : {}", Skill::Melee.to_string());
              } else {
                txt += &format!("{i} : {}", Skill::Melee.to_string()).color(Color::DarkGray).to_string();
                txt += &format!(" ({})", "无合法目标".color(Color::DarkGray));
              }
            },
            Err(msg) => {
              txt += &format!("{i} : {}", Skill::Melee.to_string()).color(Color::DarkGray).to_string();
              txt += &format!(" ({})", msg.color(Color::Red));
            },
          }
        },
        Catagory::Shoot => {
          if skills.contains(&Skill::Shoot) {
            match unit.can_skill_or_reason(&Skill::Shoot) {
              Ok(msg) => {
                if Skill::Shoot.find_target(self, id).len() > 0 {
                  valid_i.push(i);
                  txt += &format!("{i} : {}", Skill::Shoot.to_string());
                  txt += &format!(" ({})", msg);
                } else {
                  txt += &format!("{i} : {}", Skill::Shoot.to_string()).color(Color::DarkGray).to_string();
                  txt += &format!(" ({})", msg);
                  txt += &format!(" ({})", "无合法目标".color(Color::DarkGray));
                }
              },
              Err(msg) => {
                txt += &format!("{i} : {}", Skill::Shoot.to_string()).color(Color::DarkGray).to_string();
                txt += &format!(" ({})", msg.color(Color::Red));
              },
            }
          } else {
            txt += &format!("{i} : {}", Skill::Shoot.to_string()).color(Color::DarkGray).to_string();
            txt += &format!(" ({})", "无射击能力".color(Color::DarkGray));
          }
        },
        Catagory::Special => {
          valid_i.push(i);
          txt += &format!("{i} : 特技");
        },
        Catagory::Rope => {
          valid_i.push(i);
          txt += &format!("{i} : 绳索");
        },
        Catagory::Dash => {
          match unit.can_skill_or_reason(&Skill::Dash) {
            Ok(_) => {
              if Skill::Dash.find_target(self, id).len() > 0 {
                valid_i.push(i);
                txt += &format!("{i} : {}", Skill::Dash.to_string());
              } else {
                txt += &format!("{i} : {}", Skill::Dash.to_string()).color(Color::DarkGray).to_string();
                txt += &format!(" ({})", "无合法目标".color(Color::DarkGray));
              }
            },
            Err(msg) => {
              txt += &format!("{i} : {}", Skill::Dash.to_string()).color(Color::DarkGray).to_string();
              txt += &format!(" ({})", msg.color(Color::Red));
            },
          }
        },
        Catagory::Wait => {
          valid_i.push(i);
          txt += &format!("{i} : 等待");
        },
        _ => unreachable!(),
      }
      txt += "\n";
    }
    println!("{txt}");
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if ops == "\n" {
        ops = "0".to_string();
      }
      if let Ok(op) = ops.trim().parse::<usize>() {
        if valid_i.contains(&op) {
          println!("");
          return Catagory::all().get(op).unwrap().clone()
        } else {
          println!("请输入可执行的序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }

  pub fn choose_skill_with_catagory(&self, id : Id, catagory : Catagory) -> Option<Skill> {
    match catagory {
      Catagory::Melee => return Some(Skill::Melee),
      Catagory::Shoot => return Some(Skill::Shoot),
      Catagory::Dash => return Some(Skill::Dash),
      Catagory::Wait => return Some(Skill::Wait),
      _ => (),
    }
    let unit = self.id2unit(id);
    let mut txt = String::new();
    let mut valid_i = Vec::new();
    let skills : Vec<&Skill> = unit.skills().iter().filter(|skl| skl.catagory() == catagory).collect();
    let len = skills.len();
    for (i, skill) in skills.iter().enumerate() {
      match unit.can_skill_or_reason(skill) {
        Ok(msg) => {
          if skill.is_no_target() || skill.find_target(self, id).len() > 0 {
            valid_i.push(i);
            txt += &format!("{i} : {}", skill.to_string());
            if msg != "" {
              txt += &format!(" ({})", msg);
            }
          } else {
            txt += &format!("{i} : {}", skill.to_string()).color(Color::DarkGray).to_string();
            if msg != "" {
              txt += &format!(" ({})", msg);
            }
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
    txt += &format!("{len} : 返回上一级");
    println!("{}", txt);
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if ops == "\n" {
        ops = "0".to_string();
      }
      if let Ok(op) = ops.trim().parse::<usize>() {
        if op == len {
          return None;
        }
        if valid_i.contains(&op) {
          println!("");
          return Some((*skills.get(op).unwrap()).clone());
        } else {
          println!("请输入可执行的序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }

  pub fn choose_skill_with_bound(&self, id : Id) -> Skill {
    let skills = vec!(Skill::Struggle, Skill::Wait);
    let mut txt = String::new();
    let unit = self.id2unit(id);
    let mut valid_i = Vec::new();
    for (i, skill) in skills.iter().enumerate() {
      match unit.can_skill_or_reason(skill) {
        Ok(_) => {
          if skill.is_no_target() || skill.find_target(self, id).len() > 0 {
            valid_i.push(i);
            txt += &format!("{i} : {}", skill.to_string());
            if skill == &Skill::Struggle {
              txt += &format!("(束{} -> {})", unit.bound(), 0.max(unit.bound() - unit.struggle));
            }
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
          return skills.get(op).unwrap().clone()
        } else {
          println!("请输入可执行的序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }
  
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
          println!("请输入可执行的序号");
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
          let unit = self.id2unit(*id);
          let tar = self.id2unit(*idt);
          txt += &format!("{i} : {}{}", tar.colored_name(), tar.hp_bar());
          if skill.belong_to_melee(){
            let be = self.melee_expect(*id, *idt, skill);
            if be.is_back {
              txt += &"背刺".color(Color::Red).bold().to_string();
            }
            let hit = be.hit;
            let cri = be.cri;
            let dmg = be.dmg;
            txt += &format!("命{}%,伤{}~{},暴{}%", hit.to_string(), dmg, 2*dmg, cri);
          }
          if skill.belong_to_shoot() {
            let be = self.shoot_expect(*id, *idt, skill);
            if be.is_back {
              txt += &"背刺".color(Color::Red).bold().to_string();
            }
            let hit = be.hit;
            let cri = be.cri;
            let dmg = be.dmg;
            txt += &format!("命{}%,伤{}~{},暴{}%", hit.to_string(), dmg, 2*dmg, cri);
          }
          if skill == &Skill::Subdue {
            txt += &format!("(束 -> {})", unit.tie);
          }
          if skill == &Skill::Rescue {
            txt += &format!("(束{} -> {})", tar.bound(), 0.max(tar.bound() - unit.rescue));
          }
          if skill == &Skill::SecureBound {
            let f = 5.min(tar.bound() + unit.tie);
            let f = if f == 5 {"5".color(Color::Orange1).to_string()} else {f.to_string()};
            txt += &format!("(束{} -> {})", tar.bound(), f);
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
          println!("请输入可执行的序号");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }

  pub fn choose_skill_ai(&self, id : Id, rng : &mut ThreadRng) -> Skill {
    let unit = self.id2unit(id);
    let mut tiers = vec!(Vec::new(); 5);

    
    for skill in unit.skills() {
      if unit.can_skill(skill) {
        if skill.is_no_target() || skill.find_target(self, id).len() > 0 {
          let skill = skill.clone();
          match skill {
            Skill::Subdue => tiers[0].push(skill),
            Skill::Shoot => tiers[2].push(skill),
            Skill::Melee => tiers[3].push(skill),
            Skill::Wait => tiers[4].push(skill),
            Skill::Surrender => {},
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

