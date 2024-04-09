mod data_area;
pub mod io;
mod show_board;
mod turn;

use super::unit::Unit;
use crate::game::common::*;
use rand::prelude::*;
use std::collections::HashMap;

pub struct Board {
    units: Vec<Unit>,
    t: f64,
    names: HashMap<String, usize>,
    enemy_is_ai : bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            t: 0.,
            names: HashMap::new(),
            enemy_is_ai : true,
        }
    }

    pub fn add_unit(&mut self, mut unit: Unit) {
        fn usize2letter(i: usize) -> &'static str {
            // transform integer to letter ABCDE...
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".get(i..i+1).unwrap()
        }
        let mut name = unit.name.clone();
        if self.names.contains_key(&name) {
            if *self.names.get(&name).unwrap() == 0 {
                for u in self.units.iter_mut() {
                    if u.name == name.clone() + " " {
                        u.name = u.name[..u.name.len()-1].to_string() + "A";
                    }
                }
            }
            *self.names.get_mut(&name).unwrap() += 1;
            name += usize2letter(*self.names.get(&name).unwrap());
            unit.name = name;
        } else {
            self.names.insert(name.clone(), 0);
            unit.name += " ";
        }
        self.units.push(unit);
    }

    pub fn play(&mut self) {
        let t_limit = 100.0;
        let r = self.main_loop(t_limit);
        match r {
            ResultBoard::Win => println!("我方胜利！"),
            ResultBoard::Lose => println!("我方失败！"),
            ResultBoard::OutOfTime => println!("时间不足！"),
            ResultBoard::Panic => println!("出错！"),
        }
    }

    pub fn main_loop(&mut self, t_limit: f64) -> ResultBoard {
        let mut rng = thread_rng();
        while self.t < t_limit {
            let next_id = self.next_id();
            if let Some((id, t)) = next_id {
                self.t_pass(t, id);
                self.main_turn(id, &mut rng, true);
            } else {
                self.show();
                return ResultBoard::Panic;
            }
            if self.team_lose(Team::Ally) {
                self.show();
                return ResultBoard::Lose;
            }
            if self.team_lose(Team::Enemy) {
                self.show();
                return ResultBoard::Win;
            }
        }
        ResultBoard::OutOfTime
    }

    fn team_lose(&self, team: Team) -> bool {
        for unit in &self.units {
            if unit.team == team && !unit.is_bound() {
                return false;
            }
        }
        true
    }

    fn t_pass(&mut self, t: f64, id: Id) {
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
        id.map(|id| (id, least_t.unwrap()))
    }

    pub fn valid_pos(&self, pos: Pos) -> bool {
        pos >= 0 && pos < self.units.len() as i32
    }

    pub fn remove_insert(&mut self, p1: Pos, p2: Pos) {
        let p1: usize = p1.try_into().unwrap();
        let p2: usize = p2.try_into().unwrap();
        let unit = self.units.remove(p1);
        self.units.insert(p2, unit);
    }

    pub fn id2pos(&self, id: Id) -> Pos {
        for (pos, unit) in self.units.iter().enumerate() {
            if unit.id == id {
                return pos as i32;
            }
        }
        unreachable!()
    }

    pub fn pos2id(&self, pos: Pos) -> Id {
        self.pos2unit(pos).id
    }

    pub fn pos2unit_mut(&mut self, pos: Pos) -> &mut Unit {
        let pos: usize = pos.try_into().unwrap();
        &mut self.units[pos]
    }

    pub fn pos2unit(&self, pos: Pos) -> &Unit {
        let pos: usize = pos.try_into().unwrap();
        &self.units[pos]
    }

    pub fn id2unit_mut(&mut self, id: Id) -> &mut Unit {
        for unit in self.units.iter_mut() {
            if unit.id == id {
                return unit;
            }
        }
        unreachable!()
    }

    pub fn id2unit(&self, id: Id) -> &Unit {
        for unit in self.units.iter() {
            if unit.id == id {
                return unit;
            }
        }
        unreachable!()
    }
}
