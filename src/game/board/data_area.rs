use super::super::board::Board;
use super::super::unit::Unit;

impl Board {
  pub fn new_1v1_a() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_yelin(0));
    b.add_unit(Unit::new_fighter(1));
    b
  }
  
  pub fn new_4v4_a() -> Self {
    let mut b = Self::new();
    // b.add_unit(Unit::new_noal(1));
    b.add_unit(Unit::new_elis(2));
    b.add_unit(Unit::new_alyssa(3));
    b.add_unit(Unit::new_yelin(4));
    b.add_unit(Unit::new_fighter(5));
    b.add_unit(Unit::new_fighter(6));
    b.add_unit(Unit::new_arc(7));
    b.add_unit(Unit::new_arc(8));
    b
  }

  pub fn new_test_a() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_alyssa(1));
    b.add_unit(Unit::new_test_a(2));
    b.add_unit(Unit::new_fighter(5));
    b
  }

  pub fn new_test_b() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_test_b(1));
    b.add_unit(Unit::new_test_b(2));
    b.add_unit(Unit::new_fighter(5));
    b.add_unit(Unit::new_fighter(6));
    b.add_unit(Unit::new_fighter(7));
    b
  }
}