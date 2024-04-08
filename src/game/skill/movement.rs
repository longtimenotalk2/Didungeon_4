use crate::game::common::*;
use crate::game::unit::Unit;
use crate::game::board::Board;

impl Board {
  pub fn dash_to(&mut self, id1 : Id, id2 : Id) {
    let pos1 = self.id2pos(id1);
    let mut pos2 = self.id2pos(id2);
    let dir = if pos2 < pos1 {
      pos2 += 1;
      Dir::Left
    } else if pos2 > pos1 {
      pos2 -= 1;
      Dir::Right
    } else {
      Dir::None
    };
    self.pos2unit_mut(pos1).set_dir(dir);
    self.remove_insert(pos1, pos2);
  }
}
