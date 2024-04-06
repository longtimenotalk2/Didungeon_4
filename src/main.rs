pub mod game;

use crate::game::skill::battle::*;
use rand::prelude::*;

fn main() {
    // println!("{}", hit(10, 130));

    // let mut rng = thread_rng();
    // let a = rng.gen_range(1..=3);
    // println!("{}", a);

    // use game::unit::Unit;
    // let noel = Unit::new_noal(1);
    // noel.show();

    use game::board::Board;
    let b = Board::new_4v4A();
    b.show();
}