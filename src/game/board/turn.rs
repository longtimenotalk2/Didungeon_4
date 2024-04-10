use super::super::board::Board;
use crate::game::common::*;
use crate::game::skill::{Skill, Catagory};
use rand::prelude::*;


struct SkillComplete {
  id : Id, 
  skill : Skill,
  target : Option<Target>,
}

impl Board {
  pub fn main_turn(&mut self, id : Id, rng : &mut ThreadRng , show : bool) {
    self.id2unit_mut(id).turn_start();
    self.show();
    self.show_at_order();
    println!("");
    if show {
      println!("<{} 的回合>", self.id2unit(id).colored_name())
    }
    println!("");
    let ai = self.enemy_is_ai && self.id2unit(id).team == Team::Enemy;
    let sc = if ai {
      self.get_skill_complete_ai(id, rng)
    } else {
      self.get_skill_complete(id)
    };
    match sc.skill {
      Skill::Melee => {
        self.melee_exe(id, sc.target.unwrap().to_id().unwrap(), rng, show);
      },
      Skill::Shoot => {
        self.shoot_exe(id, sc.target.unwrap().to_id().unwrap(), rng, show);
      }
      Skill::Subdue => {
        self.subdue_exe(id, sc.target.unwrap().to_id().unwrap(), show)
      },
      Skill::Struggle => {
        self.struggle_exe(id, show);
      },
      Skill::Rescue => {
        self.rescue_exe(id, sc.target.unwrap().to_id().unwrap(), show)
      },
      Skill::Dash => {
        self.dash_exe(id, &sc.target.unwrap());
      }
      Skill::Wait => {
        self.wait_exe(id, show);
      }
    }
  }

  fn get_skill_complete(&self, id : Id) -> SkillComplete {
    loop {
      let catagory = self.choose_catagory(id);
      loop {
        let skill = self.choose_skill_with_catagory(id, catagory);
        if let Some(skill) = skill {
          let targets = skill.find_target(self, id);
          if targets.len() == 0 {
            return SkillComplete {
              id,
              skill,
              target : None,
            }
          } else {
            if let Some(target) = self.choose_target(&id, &skill, &targets) {
              return SkillComplete {
                id,
                skill,
                target : Some(target),
              }
            } else {
              match catagory {
                Catagory::Melee => break,
                Catagory::Shoot => break,
                Catagory::Special => continue,
                Catagory::Rope => continue,
                Catagory::Dash => break,
                Catagory::Wait => break,
              }
            }
          }
        } else {
          break
        }
      }
    }
  }



  fn get_skill_complete_ai(&self, id : Id, rng : &mut ThreadRng)  -> SkillComplete {
    let skill = self.choose_skill_ai(id, rng);
    let targets = skill.find_target(self, id);
    if targets.len() == 0 {
      SkillComplete {
        id,
        skill,
        target : None,
      }
    } else {
      let target = targets.choose(rng).unwrap().clone();
      SkillComplete {
        id,
        skill,
        target : Some(target),
      }
    }
  }

  fn show_at_order(&self) {
    let mut txt = String::new();
    for id in self.at_order() {
      txt += &self.id2unit(id).colored_name();
      txt += "|";
    }
    txt = txt[..txt.len()-1].to_string();
    println!("{}", txt);
  }

  fn at_order(&self) -> Vec<Id> {
    let mut ats = Vec::new();
    for unit in self.units.iter() {
      let t0 = unit.calc_t();
      ats.push((t0, unit.id));
    }
    ats.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    ats.into_iter().map(|(_, id)| id).collect()
  }
}


