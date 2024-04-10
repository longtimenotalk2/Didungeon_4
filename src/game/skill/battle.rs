use crate::game::common::*;
use crate::game::board::Board;
use rand::prelude::*;
use colorful::Color;
use colorful::Colorful;


pub struct BattleExpect {
  pub hit : i32,
  pub dmg : i32,
  pub cri : i32,
  pub is_back : bool,
}

impl Board {
  pub fn melee_expect(&self, id1 : Id, id2 : Id) -> BattleExpect {
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let dir = self.dir_to(id1, id2);
    let is_back = dir == tar.dir();
    let atk = unit.atk_melee();
    let mut def = tar.def_melee();
    if is_back {
      def *= 0.5;
    }
    let dmg = dmg(atk, def);
    let acc = unit.dex();
    let mut evd = tar.agi() * 0.5 + tar.dex() * 0.25 + tar.luck() * 0.25;
    if is_back {
      evd *= 0.5;
    }
    let hit = hit(acc, evd);
    let crieff = unit.dex() * 0.5 + unit.luck() * 0.5;
    let res = tar.luck();
    let mut base_cri = 0;
    if is_back {
      base_cri += 40;
    }
    let cri = effect_hit(crieff, res, base_cri);
    let expect = BattleExpect {
      hit,
      dmg,
      cri,
      is_back,
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
    if is_hit {
      let dir = self.dir_to(id1, id2);
      self.id2unit_mut(id2).set_dir(dir.anti())
    }
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let tar = self.id2unit(id2);
      let back = if expect.is_back {
        "背刺! ".color(Color::Red).bold().to_string()
      } else {"".to_string()};
      if is_hit {
        if is_cri {
          println!("====> {}{} <==== (挥击 {})", back, format!("暴击! {dmg}!").color(Color::Orange1).bold(), tar.colored_name());
        } else {
          println!("====> {}{} <==== (挥击 {})", back, format!("{dmg}!").color(Color::Red).bold(), tar.colored_name());
        }
      } else {
        println!("====> {}{} <==== (挥击 {})", back,  "Miss".color(Color::BlueViolet).bold(),  tar.colored_name());
      }
      println!("");
    }
  }

  pub fn shoot_expect(&self, id1 : Id, id2 : Id) -> BattleExpect {
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let dir = self.dir_to(id1, id2);
    let is_back = dir == tar.dir();
    let atk = unit.atk_shoot();
    let mut def = tar.def_shoot();
    if is_back {
      def *= 0.5;
    }
    let dmg = dmg(atk, def);
    let acc = unit.dex();
    let mut evd = tar.agi() * 0.5 + tar.dex() * 0.25 + tar.luck() * 0.25;
    if is_back {
      evd *= 0.5;
    }
    let hit = hit(acc, evd);
    let crieff = unit.dex() * 0.5 + unit.luck() * 0.5;
    let res = tar.luck();
    let mut base_cri = 0;
    if is_back {
      base_cri += 40;
    }
    let cri = effect_hit(crieff, res, base_cri);
    let expect = BattleExpect {
      hit,
      dmg,
      cri,
      is_back,
    };
    expect
  }

  pub fn shoot_exe(&mut self, id1 : Id, id2 : Id, rng : &mut ThreadRng , show : bool) {
    let expect = self.shoot_expect(id1, id2);
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
    let dir = self.dir_to(id1, id2);
    self.id2unit_mut(id1).set_dir(dir);
    if is_hit {
      let dir = self.dir_to(id1, id2);
      self.id2unit_mut(id2).set_dir(dir.anti())
    }
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let tar = self.id2unit(id2);
      let back = if expect.is_back {
        "背刺! ".color(Color::Red).bold().to_string()
      } else {"".to_string()};
      if is_hit {
        if is_cri {
          println!("====> {}{} <==== (射击 {})", back, format!("暴击! {dmg}!").color(Color::Orange1).bold(), tar.colored_name());
        } else {
          println!("====> {}{} <==== (射击 {})", back, format!("{dmg}!").color(Color::Red).bold(), tar.colored_name());
        }
      } else {
        println!("====> {}{} <==== (射击 {})", back,  "Miss".color(Color::BlueViolet).bold(),  tar.colored_name());
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



