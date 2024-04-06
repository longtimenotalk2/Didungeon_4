mod turn;

use super::unit::Unit;
use crate::game::common::*;

pub struct Board {
  units : Vec<Unit>,
  t : f64,
}

impl Board {
  pub fn new() -> Self {
    Self {
      units : Vec::new(),
      t : 0.,
    }
  }

  pub fn add_unit(&mut self, unit : Unit) {
    self.units.push(unit);
  }

  pub fn main_loop(&mut self, t_limit : f64) -> ResultBoard {
    while self.t < t_limit {
      let next_id = self.next_id();
      if let Some((id, t)) = next_id {
        self.t_pass(t, id);
        self.main_turn(id);
      } else {
        return ResultBoard::Panic;
      }
      if self.team_lose(Team::Ally) {
        return ResultBoard::Lose;
      }
      if self.team_lose(Team::Enemy) {
        return ResultBoard::Win;
      }
    }
    ResultBoard::OutOfTime
  }

  fn team_lose(&self, team : Team) -> bool {
    for unit in &self.units {
      if unit.team == team && !unit.is_bound() {
        return false;
      }
    }
    true
  }

  fn t_pass(&mut self, t : f64, id : Id) {
    for unit in self.units.iter_mut() {
      let is_this = id == unit.id;
      unit.t_pass(t, is_this);
    }
    self.t += t;
  }

  fn next_id(&self) -> Option<(Id, f64)> {
    let mut least_t = None;
    let mut id = None;
    for unit in self.units.iter() {
      let t = unit.calc_t();
      if least_t.is_none() {
        least_t = Some(t);
        id = Some(unit.id);
      } else {
        if t < least_t.unwrap() {
          least_t = Some(t);
          id = Some(unit.id);
        }
      }
    }
    id.map(|id|(id, least_t.unwrap()))
  }
}