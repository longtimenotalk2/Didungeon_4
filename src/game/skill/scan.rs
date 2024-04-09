use crate::game::common::*;
use crate::game::board::Board;
use crate::game::skill::Skill;

impl Skill {
  pub fn find_target(&self, board : &Board, id : Id) -> Vec<Target> {
    match self {
      Skill::Melee => board.find_melee_option(id, 1).iter().map(|t| Target::Single(*t)).collect(),
      Skill::Subdue => board.find_subdue_option(id).iter().map(|t| Target::Single(*t)).collect(),
      Skill::Rescue => board.find_rescue_option(id).iter().map(|t| Target::Single(*t)).collect(),
      _ => vec!(),
    }
  }
}

impl Board {
  fn find_melee_option(&self, id : Id, bypass : i32) -> Vec<Id> {
    let team = self.id2unit(id).team;
    let mut list = Vec::new();
    for (i, scan) in self.scan(id).iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        if tar.team != team && !tar.is_bound() && bypass >= scan.block {
          list.push(tar.id);
        }
      }
    }
    list
  }

  fn find_subdue_option(&self, id : Id) -> Vec<Id> {
    let team = self.id2unit(id).team;
    let mut list = Vec::new();
    for (i, scan) in self.scan(id).iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        if tar.team != team && !tar.is_bound() && tar.is_weak() && scan.block <= 0 {
          list.push(tar.id);
        }
      }
    }
    list
  }

  fn find_rescue_option(&self, id : Id) -> Vec<Id> {
    let team = self.id2unit(id).team;
    let mut list = Vec::new();
    for (i, scan) in self.scan(id).iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        if tar.team == team && tar.is_bound() && scan.block <= 0 {
          list.push(tar.id);
        }
      }
    }
    list
  }
}

struct Scan {
  dir : Dir,
  dist : i32,
  block : i32,
}

impl Board {
  fn scan(&self, id : Id) -> Vec<Option<Scan>> {
    let pos = self.id2pos(id);
    let unit = self.pos2unit(pos);
    let team = unit.team;
    let mut scan_left = Vec::new();
    let mut dist = 1;
    let mut block = 0;
    while self.valid_pos(pos - dist) {
      let p = pos - dist;
      let tar = self.pos2unit(p);
      scan_left.push(
        Some(Scan {
          dir : Dir::Left,
          dist,
          block,
        }
      ));
      if tar.team != team && tar.can_block(){
        block += 1;
      }
      dist += 1;
    }
    
    let mut scan_right = Vec::new();
    let mut dist = 1;
    let mut block = 0;
    while self.valid_pos(pos + dist) {
      let p = pos + dist;
      let tar = self.pos2unit(p);
      scan_right.push(
        Some(Scan {
          dir : Dir::Right,
          dist,
          block,
        }
      ));
      if tar.team != team && tar.can_block(){
        block += 1;
      }
      dist += 1;
    }

    // reverse scan_left and concatenate scan_right
    scan_left.reverse();
    scan_left.push(None);
    scan_left.extend(scan_right);
    scan_left
  }
}

