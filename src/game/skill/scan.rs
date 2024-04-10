use crate::game::common::*;
use crate::game::board::Board;
use crate::game::skill::Skill;
use crate::game::unit::Unit;

enum TargetTeamType {
  Enemy,
  Ally,
}

struct ReachDetail {
  bypass : i32,
  ttt : TargetTeamType,
  demand : Box<dyn Fn(&Unit) -> bool>,
}

struct RangeDetail {
  pierce : i32,
  ttt : TargetTeamType,
  demand : Box<dyn Fn(&Unit) -> bool>,
}

impl Skill {
  pub fn find_target(&self, board : &Board, id : Id) -> Vec<Target> {
    match self {
      Skill::Melee => {
        let detail = ReachDetail {
          bypass : 1,
          ttt : TargetTeamType::Enemy,
          demand : Box::new(|tar : &Unit| 
            !tar.is_bound()
          ),
        };
        board.find_reach_option(id, &detail)
      },
      Skill::Shoot => {
        let detail = RangeDetail {
          pierce : 2,
          ttt : TargetTeamType::Enemy,
          demand : Box::new(|tar : &Unit| 
            !tar.is_bound()
          ),
        };
        board.find_range_option(id, &detail)
      }
      Skill::Subdue => {
        let detail = ReachDetail {
          bypass : 0,
          ttt : TargetTeamType::Enemy,
          demand : Box::new(|tar : &Unit| 
            !tar.is_bound() && tar.is_weak()
          ),
        };
        board.find_reach_option(id, &detail)
      },
      Skill::Struggle => Vec::new(),
      Skill::Rescue => {
        let detail = ReachDetail {
          bypass : 0,
          ttt : TargetTeamType::Ally,
          demand : Box::new(|tar : &Unit| 
            tar.is_bound()
          ),
        };
        board.find_reach_option(id, &detail)
      },
      Skill::Dash => board.find_dash_option(id, 1),
      Skill::Wait => Vec::new(),
    }
  }
}

impl Board {
  fn find_reach_option(&self, id : Id, detail : &ReachDetail) -> Vec<Target> {
    let unit = self.id2unit(id);
    let team = unit.team;
    let mut list = Vec::new();
    for (i, scan) in self.scan(id).iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        let bypass = if scan.zoc {0} else {detail.bypass};
        let team_match = match detail.ttt {
          TargetTeamType::Enemy => team != tar.team,
          TargetTeamType::Ally => team == tar.team,
        };
        if team_match && bypass >= scan.block && (detail.demand)(&tar) {
          list.push(Target::Single(tar.id));
        }
      }
    }
    list
  }
  
  fn find_range_option(&self, id : Id, detail : &RangeDetail) -> Vec<Target> {
    let unit = self.id2unit(id);
    let team = unit.team;
    let mut list = Vec::new();
    for (i, scan) in self.scan(id).iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        let pierce = detail.pierce;
        let team_match = match detail.ttt {
          TargetTeamType::Enemy => team != tar.team,
          TargetTeamType::Ally => team == tar.team,
        };
        if team_match && !scan.zoc && scan.dist > 1 && pierce >= scan.block && (detail.demand)(&tar) {
          list.push(Target::Single(tar.id));
        }
      }
    }
    list
  }

  fn find_dash_option(&self, id : Id, bypass : i32) -> Vec<Target> {
    let team = self.id2unit(id).team;
    let mut list = Vec::new();
    let scans = self.scan(id);
    if let Some(scan) = scans.get(0).unwrap() {
      let tar = self.pos2unit(0);
      let bypass = if scan.zoc {0} else {bypass};
      if bypass >= scan.block_with_this {
        list.push(Target::Border(Dir::Left));
      }
    }
    for (i, scan) in scans.iter().enumerate() {
      if let Some(scan) = scan {
        let pos = i as i32;
        let tar = self.pos2unit(pos);
        let bypass = if scan.zoc {0} else {bypass};
        if scan.dist > 1 && bypass >= scan.block {
          list.push(Target::Single(tar.id))
        }
      }
    }
    let last_index = scans.len() - 1;
    if let Some(scan) = scans.get(last_index).unwrap() {
      let tar = self.pos2unit(last_index as i32);
      let bypass = if scan.zoc {0} else {bypass};
      if bypass >= scan.block_with_this {
        list.push(Target::Border(Dir::Right));
      }
    }
    list
  }
}

#[derive(Debug)]
struct Scan {
  dir : Dir,
  dist : i32,
  block : i32,
  zoc : bool,
  block_with_this : i32,
}

impl Board {
  fn scan(&self, id : Id) -> Vec<Option<Scan>> {
    let pos = self.id2pos(id);
    let unit = self.pos2unit(pos);
    let team = unit.team;
    let mut scan_left = Vec::new();
    let mut dist = 1;
    let mut block = 0;
    let mut zoc = false;
    while self.valid_pos(pos - dist) {
      let p = pos - dist;
      let tar = self.pos2unit(p);
      if dist == 1 && tar.zoc().contains(&Dir::Right) {
        zoc = true;
      }
      let block_old = block;
      if tar.team != team && tar.can_block(){
        block += 1;
      }
      scan_left.push(
        Some(Scan {
          dir : Dir::Left,
          dist,
          block : block_old,
          zoc,
          block_with_this : block,
        }
      ));
      dist += 1;
    }
    
    let mut scan_right = Vec::new();
    let mut dist = 1;
    let mut block = 0;
    let mut zoc = false;
    while self.valid_pos(pos + dist) {
      let p = pos + dist;
      let tar = self.pos2unit(p);
      if dist == 1 && tar.team != team && tar.zoc().contains(&Dir::Left) {
        zoc = true;
      }
      let block_old = block;
      if tar.team != team && tar.can_block(){
        block += 1;
      }
      scan_right.push(
        Some(Scan {
          dir : Dir::Left,
          dist,
          block : block_old,
          zoc,
          block_with_this : block,
        }
      ));
      dist += 1;
    }

    // reverse scan_left and concatenate scan_right
    scan_left.reverse();
    scan_left.push(None);
    scan_left.extend(scan_right);
    scan_left
  }
}

