use crate::game::common::*;
use crate::game::unit::Unit;
use crate::game::board::Board;

impl Board {
  pub fn wait_exe(&mut self, id : Id, show : bool) {
    let unit = self.id2unit_mut(id);
    unit.at_delay(50.);
    if show {
      println!("{} 等待", unit.colored_name());
      println!("")
    }
  }

  pub fn dash_exe(&mut self, id1 : Id, t : &Target) {
    let unit = self.id2unit_mut(id1);
    unit.at_delay(100.);
    match t {
      Target::Single(id2) => {
        self.dash_to(id1, *id2);
      },
      Target::Border(dir) => {
        let pos1 = self.id2pos(id1);
        match dir {
          Dir::Left => self.remove_insert(pos1, 0),
          Dir::Right => self.remove_insert(pos1, self.len() as Pos),
          _ => unreachable!(),
        }
      },
    }
  }
  
  pub fn dash_to(&mut self, id1 : Id, id2 : Id) {
    let pos1 = self.id2pos(id1);
    let mut pos2 = self.id2pos(id2);
    if pos2 < pos1 {
      pos2 += 1;
    } else if pos2 > pos1 {
      pos2 -= 1;
    }
    self.remove_insert(pos1, pos2);
  }
}
