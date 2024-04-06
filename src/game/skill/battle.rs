use crate::game::common::*;
use crate::game::unit::Unit;
use rand::prelude::*;


struct BattleExpect {
  hit : i32,
  dmg : i32,
}

impl Unit {
  pub fn melee_expect(&self, tar : &Unit) -> BattleExpect {
    let atk = self.atk_melee;
    let def = tar.def_melee;
    let dmg = dmg(atk, def);
    let acc = self.dex;
    let evd = tar.dex;
    let hit = hit(acc, evd);
    let expect = BattleExpect {
      hit,
      dmg,
    };
    expect
  }

  pub fn melee_exe(&self, tar : &mut Unit, rng : &mut ThreadRng , show : bool) {
    let expect = self.melee_expect(tar);
    let hit = expect.hit;
    let dmg = expect.dmg;
    let is_hit = rng.gen_range(1..=100) <= hit;
    if is_hit {
      tar.take_dmg(dmg);
    }
    if show {
      if is_hit {
        println!("{} 攻击 {}， {} !", self.colored_name(), tar.colored_name(), dmg);
      } else {
        println!("{} 攻击 {}， 落空", self.colored_name(), tar.colored_name());
      }
    }
  }
    
}

fn dmg(atk : i32, def : i32) -> i32 {
  let atk = atk as f64;
  let def = def as f64;
  let dmg = atk * atk / (atk + def);
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

