use crate::game::common::*;
use crate::game::board::Board;
use rand::prelude::*;


pub struct BattleExpect {
  pub hit : i32,
  pub dmg : i32,
}

impl Board {
  pub fn melee_expect(&self, id1 : Id, id2 : Id) -> BattleExpect {
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let atk = unit.atk_melee;
    let def = tar.def_melee;
    let dmg = dmg(atk, def);
    let acc = unit.dex;
    let evd = tar.dex;
    let hit = hit(acc, evd);
    let expect = BattleExpect {
      hit,
      dmg,
    };
    expect
  }

  pub fn melee_exe(&mut self, id1 : Id, id2 : Id, rng : &mut ThreadRng , show : bool) {
    let expect = self.melee_expect(id1, id2);
    let hit = expect.hit;
    let dmg = expect.dmg;
    let is_hit = rng.gen_range(1..=100) <= hit;
    if is_hit {
      self.id2unit_mut(id2).take_dmg(dmg);
    }
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      if is_hit {
        println!("{} 挥击 {}， {} !", unit.colored_name(), tar.colored_name(), dmg);
      } else {
        println!("{} 挥击 {}， 落空", unit.colored_name(), tar.colored_name());
      }
      println!("");
    }
  }
    
}

fn dmg(atk : i32, def : i32) -> i32 {
  let atk = atk as f64;
  let def = def as f64;
  let dmg = 3.0 * atk * atk / (atk + def);
  dmg as i32
}

pub fn hit(acc : i32, evd : i32) -> i32 {
  use std::f64::consts::PI;
  let c = 0.85;
  let z = (1./c - 1.) / PI * 2.0;
  let a = acc as f64;
  let e = evd as f64;
  let q = (a - e) / (10. + a + e);
  let r = if q < 0.0 {
    c * (1.0 + q)
  } else {
    c * (1.0 + z * (q / z).atan())
  };
  (r * 100.) as i32
}

