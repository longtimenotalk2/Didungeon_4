pub mod game;
use game::board::Board;



fn main() {
    
    // let mut b = Board::new_1v1_a();
    let mut b = Board::new_4v4_a();
    // let mut b = Board::new_test_a();
    // let mut b = Board::new_test_b();
    // let mut b = Board::new_test_c();
    
    b.play();


}