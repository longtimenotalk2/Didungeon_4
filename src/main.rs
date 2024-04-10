pub mod game;

// use crate::game::skill::battle::*;
// use rand::prelude::*;
// use colorful::Color;
// use colorful::Colorful;


fn main() {
    // println!("{}", hit(10, 130));

    // let mut rng = thread_rng();
    // let a = rng.gen_range(1..=3);
    // println!("{}", a);

    // use game::unit::Unit;
    // let noel = Unit::new_noal(1);
    // noel.show();

    use game::board::Board;
    // let mut b = Board::new_1v1_a();
    // let mut b = Board::new_4v4_a();
    let mut b = Board::new_test_a();
    // let mut b = Board::new_test_b();
    
    b.play();

    //给文字增添删除线

    // let s = "ss".color(Color::Red).dim();
    // print!("{s}");

}