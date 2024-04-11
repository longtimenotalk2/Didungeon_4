use crate::game::common::*;
use crate::game::board::Board;
use rand::prelude::*;
use colorful::Color;
use colorful::Colorful;
use crate::game::skill::Skill;

struct MeleeCard {
  atk_rate_fix : f64,
  acc_rate_fix : f64,
  base_cri : i32,
}

struct ShootCard {
  atk_rate_fix : f64,
  acc_rate_fix : f64,
  base_cri : i32,
}

impl Skill {
  fn melee_card(&self) -> Option<MeleeCard> {
    match self {
      Skill::Melee => Some(MeleeCard {
        atk_rate_fix : 1.,
        acc_rate_fix : 1.,
        base_cri : 0,
      }),
      Skill::Xuliyiji => Some(MeleeCard {
        atk_rate_fix : 1.5,
        acc_rate_fix : 0.7,
        base_cri : 20,
      }),
      Skill::Cangyanzhihun => Some(MeleeCard {
          atk_rate_fix : 1.0,
          acc_rate_fix : 1.5,
          base_cri : 10,
        }),
      Skill::RoundhouseKick => Some(MeleeCard {
          atk_rate_fix : 1.3,
          acc_rate_fix : 0.8,
          base_cri : 30,
        }),
      Skill::Whack => Some(MeleeCard {
          atk_rate_fix : 1.3,
          acc_rate_fix : 0.9,
          base_cri : 10,
        }),
      _ => None,
    }
  }

  fn shoot_card(&self) -> Option<ShootCard> {
    match self {
      Skill::Shoot => Some(ShootCard {
        atk_rate_fix : 1.,
        acc_rate_fix : 1.,
        base_cri : 0,
      }),
      Skill::PrecisionSniping => Some(ShootCard {
        atk_rate_fix : 1.2,
        acc_rate_fix : 1.2,
        base_cri : 10,
      }),
      _ => None,
    }
  }

  pub fn belong_to_melee(&self) -> bool {
    self.melee_card().is_some()
  }

  pub fn belong_to_shoot(&self) -> bool {
    self.shoot_card().is_some()
  }
  
}

pub struct BattleExpect {
  pub hit : i32,
  pub dmg : i32,
  pub cri : i32,
  pub is_back : bool,
}

impl Board {
  pub fn melee_expect(&self, id1 : Id, id2 : Id, skill : &Skill) -> BattleExpect {
    let card = skill.melee_card().expect(&format!("技能 {} 无法被识别为Melee", skill.to_string()));
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let dir = self.dir_to(id1, id2);
    let is_back = dir == tar.dir();
    let atk = unit.atk_melee() * card.atk_rate_fix;
    let mut def = tar.def_melee();
    if is_back {
      def *= 0.5;
    }
    let dmg = dmg(atk, def);
    let acc = unit.dex() * card.acc_rate_fix;
    let mut evd = tar.agi() * 0.5 + tar.dex() * 0.25 + tar.luck() * 0.25;
    if is_back {
      evd *= 0.5;
    }
    let hit = hit(acc, evd);
    let crieff = unit.dex() * 0.5 + unit.luck() * 0.5;
    let res = tar.luck();
    let mut base_cri = card.base_cri;
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

  pub fn melee_exe(&mut self, id1 : Id, id2 : Id, skill : &Skill, rng : &mut ThreadRng , show : bool) {
    let expect = self.melee_expect(id1, id2, skill);
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
      self.id2unit_mut(id2).tp_add(10);
      self.id2unit_mut(id1).tp_add(10);
      if is_cri {
        self.id2unit_mut(id1).tp_add(10);
      }
    } else {
      self.id2unit_mut(id2).tp_add(5);
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
      let typetxt = if is_hit {
        if is_cri {
          format!("暴击! {dmg}!").color(Color::Orange1).bold()
        } else {
          format!("{dmg}!").color(Color::Red).bold()
        }
      } else {
        "Miss".color(Color::BlueViolet).bold()
      };
      println!("====> {}{} <==== ({} {}{})", back, typetxt, skill.to_string(), tar.colored_name(), tar.hp_bar());
      println!("");
    }
  }

  pub fn shoot_expect(&self, id1 : Id, id2 : Id, skill : &Skill) -> BattleExpect {
    let card = skill.shoot_card().expect(&format!("技能 {} 无法被识别为Shoot", skill.to_string()));
    let unit = self.id2unit(id1);
    let tar = self.id2unit(id2);
    let dir = self.dir_to(id1, id2);
    let is_back = dir == tar.dir();
    let atk = unit.atk_shoot() * card.atk_rate_fix;
    let mut def = tar.def_shoot();
    if is_back {
      def *= 0.5;
    }
    let dmg = dmg(atk, def);
    let acc = unit.dex() * card.acc_rate_fix;
    let mut evd = tar.agi() * 0.5 + tar.dex() * 0.25 + tar.luck() * 0.25;
    if is_back {
      evd *= 0.5;
    }
    let hit = hit(acc, evd);
    let crieff = unit.dex() * 0.5 + unit.luck() * 0.5;
    let res = tar.luck();
    let mut base_cri = card.base_cri;
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

  pub fn shoot_exe(&mut self, id1 : Id, id2 : Id, skill : &Skill, rng : &mut ThreadRng , show : bool) {
    let expect = self.shoot_expect(id1, id2, skill);
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
      self.id2unit_mut(id2).tp_add(10);
      self.id2unit_mut(id1).tp_add(10);
      if is_cri {
        self.id2unit_mut(id1).tp_add(10);
      }
    } else {
      self.id2unit_mut(id2).tp_add(5);
    }
    let dir = self.dir_to(id1, id2);
    self.id2unit_mut(id1).set_dir(dir);
    // if is_hit {
    //   let dir = self.dir_to(id1, id2);
    //   self.id2unit_mut(id2).set_dir(dir.anti())
    // }
    self.id2unit_mut(id1).at_delay(100.);
    if show {
      let tar = self.id2unit(id2);
      let back = if expect.is_back {
        "背刺! ".color(Color::Red).bold().to_string()
      } else {"".to_string()};
      let typetxt = if is_hit {
        if is_cri {
          format!("暴击! {dmg}!").color(Color::Orange1).bold()
        } else {
          format!("{dmg}!").color(Color::Red).bold()
        }
      } else {
        "Miss".color(Color::BlueViolet).bold()
      };
      println!("====> {}{} <==== ({} {}{})", back, typetxt, skill.to_string(), tar.colored_name(), tar.hp_bar());
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



