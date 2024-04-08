use crate::game::common::*;
use crate::game::board::Board;

impl Board {
  pub fn subdue_exe(&mut self, id1 : Id, id2 : Id, show : bool) {
    let unit = self.id2unit(id1);
    let tie = unit.tie;
    let tar = self.id2unit_mut(id2);
    tar.bound_add(tie);
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      println!("{} 捆绑 {} 至 {} 层", unit.colored_name(), tar.colored_name(), tie);
      println!("")
    }
  }
}