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

    pub fn new_4v4_b() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_noal(1));
    b.add_unit(Unit::new_elis(2));
    b.add_unit(Unit::new_alyssa(3));
    b.add_unit(Unit::new_yelin(4));
    b.add_unit(Unit::new_fighter(5));
    b.add_unit(Unit::new_fighter(6));
      b.add_unit(Unit::new_fighter(7));
    b.add_unit(Unit::new_arc(11));
    b.add_unit(Unit::new_arc(12));
          b.add_unit(Unit::new_arc(13));
    b
  }

  pub fn new_test_a() -> Self {
    let mut b = Self::new();
    b.add_unit(Unit::new_alyssa(1));
    b.add_unit(Unit::new_test_a(2));
    let mut f = Unit::new_fighter(4);
    f.take_dmg(500);
    f.bound_add(4);
    b.add_unit(f);
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

  pub fn new_test_c() -> Self {
    let mut arc = Unit::new_arc(3);
    arc.bound_add(3);
    let mut b = Self::new();
    b.add_unit(Unit::new_alyssa(1));
    b.add_unit(Unit::new_elis(2));
    b.add_unit(arc);
    b.add_unit(Unit::new_fighter(4));
    b
  }

  pub fn new_panic_a() -> Self {
    let mut bd1 = Unit::new_alyssa(11);
    bd1.bound_add(3);
    let mut bd2 = Unit::new_yelin(12);
    bd2.bound_add(3);
    let mut b = Self::new();
    b.add_unit(Unit::new_noal(1));
    b.add_unit(Unit::new_elis(2));
    b.add_unit(Unit::new_alyssa(3));
    // b.add_unit(Unit::new_noal(4));
    b.add_unit(Unit::new_fighter(5));
    b.add_unit(Unit::new_fighter(6));
    b.add_unit(Unit::new_arc(7));
    b.add_unit(Unit::new_arc(8));
    
    b.add_unit(bd1);
    b.add_unit(bd2);
    b
  }
}