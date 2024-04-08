use super::super::board::Board;
use crate::game::common::*;
use crate::game::skill::Skill;
use rand::prelude::*;


struct SkillComplete {
  id : Id, 
  skill : Skill,
  target : Option<Target>,
}

impl Board {
  pub fn main_turn(&mut self, id : Id, rng : &mut ThreadRng , show : bool) {
    self.show();
    println!("");
    if show {
      println!("<{} 的回合>", self.id2unit(id).colored_name())
    }
    println!("");
    let sc = self.get_skill_complete(id);
    match sc.skill {
      Skill::Melee => {
        self.melee_exe(id, sc.target.unwrap().to_id().unwrap(), rng, show);
      },
      Skill::Subdue => {
        self.subdue_exe(id, sc.target.unwrap().to_id().unwrap(), show)
      },
      _ => {},
    }
  }

  fn get_skill_complete(&self, id : Id) -> SkillComplete {
    loop {
      let skill = self.choose_skill(id);
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
          continue;
        }
      }
    }
  }
}


