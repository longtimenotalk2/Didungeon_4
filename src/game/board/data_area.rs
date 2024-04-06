use super::super::board::Board;
use super::super::unit::Unit;
use crate::game::common::*;

impl Board {
  pub fn new_4v4A() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_noal(1));
    b.add_unit(Unit::new_elis(2));
    b.add_unit(Unit::new_alyssa(3));
    b.add_unit(Unit::new_yelin(4));
    b.add_unit(Unit::new_fighter(5));
    b.add_unit(Unit::new_fighter(6));
    b.add_unit(Unit::new_thief(7));
    b.add_unit(Unit::new_thief(8));
    b
  }
}