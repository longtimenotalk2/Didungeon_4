use crate::game::common::*;
use crate::game::board::Board;
use rand::prelude::*;
use colorful::Color;
use colorful::Colorful;


pub struct BattleExpect {
  pub hit : i32,
  pub dmg : i32,
  pub cri : i32,
}

impl Board {
  pub fn melee_expect(&self, id1 : Id, id2 : Id) -> BattleExpect {
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let atk = unit.atk_melee();
    let def = tar.def_melee();
    let dmg = dmg(atk, def);
    let acc = unit.dex();
    let evd = tar.agi() * 0.5 + tar.dex() * 0.25 + tar.luck() * 0.25;
    let hit = hit(acc, evd);
    let crieff = unit.dex() * 0.5 + unit.luck() * 0.5;
    let res = tar.luck();
    let cri = effect_hit(crieff, res, 0);
    let expect = BattleExpect {
      hit,
      dmg,
      cri,
    };
    expect
  }

  pub fn melee_exe(&mut self, id1 : Id, id2 : Id, rng : &mut ThreadRng , show : bool) {
    let expect = self.melee_expect(id1, id2);
    let hit = expect.hit;
    let mut dmg = expect.dmg;
    let is_hit = rng.gen_range(1..=100) <= hit;
    let mut is_cri = false;
    if is_hit {
      is_cri = rng.gen_range(1..=100) <= expect.cri;
      if is_cri {
        dmg *= 3;
      }else {
        let y: f64 = rng.gen();
        dmg = (dmg as f64 * (1.0 + y)) as i32;
      }
      self.id2unit_mut(id2).take_dmg(dmg);
    }
    self.dash_to(id1, id2);
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let unit = self.id2unit(id1);
      let tar = self.id2unit(id2);
      if is_hit {
        if is_cri {
          println!("====> {} <==== ({} 挥击 {})", format!("暴击! {dmg}!").color(Color::Orange1).bold(), unit.colored_name(), tar.colored_name());
        } else {
          println!("====> {} <==== ({} 挥击 {})", format!("{dmg}!").color(Color::Red).bold(), unit.colored_name(), tar.colored_name());
        }
      } else {
        println!("====> {} <==== ({} 挥击 {})", "Miss".color(Color::LightGray).bold(), unit.colored_name(), tar.colored_name());
      }
      println!("");
    }
  }
    
}

fn dmg(atk : f64, def : f64) -> i32 {
  let atk = atk as f64;
  let def = def as f64;
  let dmg = atk * atk / (atk + def);
  dmg as i32
}

fn hit(acc : f64, evd : f64) -> i32 {
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

fn effect_hit(eff : f64, res : f64, base : i32) -> i32 {
  let c = 3.;
  let mut r = (eff + c * base as f64 - res) / res / c ;
  if r < 0. {r = 0.;}
  (r * 100.) as i32
}



