use crate::game::common::*;
use crate::game::board::Board;
use crate::game::buff::Buff;

impl Board {
  pub fn subdue_exe(&mut self, id1 : Id, id2 : Id, show : bool) {
    let dir = self.dir_to(id1, id2);
    let unit = self.id2unit(id1);
    let tie = unit.tie;
    let tar = self.id2unit_mut(id2);
    tar.bound_add(tie);
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).set_dir(dir);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      println!("{} 捆绑 {} 至 {} 层", unit.colored_name(), tar.colored_name(), tie);
      println!("")
    }
  }

  pub fn struggle_exe(&mut self, id : Id, show : bool) {
    let unit = self.id2unit_mut(id);
    let struggle = unit.struggle;
    unit.bound_sub(struggle);
    unit.at_delay(100.);
    if show {
      println!("{} 挣扎 {} 层，剩余 {} 层", unit.colored_name(), struggle, unit.bound());
      println!("")
    }
  }

  pub fn rescue_exe(&mut self, id1 : Id, id2 : Id, show : bool) {
    let dir = self.dir_to(id1, id2);
    let unit = self.id2unit(id1);
    let rescue = unit.rescue;
    let tar = self.id2unit_mut(id2);
    tar.bound_sub(rescue);
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).set_dir(dir);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      println!("{} 解绑 {} {} 层，剩余 {} 层", unit.colored_name(), tar.colored_name(), rescue, tar.bound());
      println!("")
    }
  }

  pub fn secure_bound_exe(&mut self, id1 : Id, id2 : Id, show : bool) {
    let dir = self.dir_to(id1, id2);
    let unit = self.id2unit(id1);
    let tie = unit.tie;
    let tar = self.id2unit_mut(id2);
    tar.bound_add(tie);
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).set_dir(dir);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      println!("{} 加固 {} {} 层 至 {} 层", unit.colored_name(), tar.colored_name(), tie, tar.bound());
      println!("")
    }
  }

  pub fn surrender_exe(&mut self, id : Id, t : &Target) {
    self.dash_exe(id, t);
    self.id2unit_mut(id).refresh_buff(Buff::Surrender, 1);
  }
}