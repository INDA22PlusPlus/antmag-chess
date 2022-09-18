
include!("lib.rs");

fn main() {
    
}

#[cfg(test)]
mod test{
    use test_crate::chess_api::Cell;

    use crate::chess_api::{Board_state, create_init_board};

    #[test]
    fn test_init_config(){
        let b : Board_state = create_init_board(false);
        let s = Cell::King; 
        println!("test!");
        ()
    }
}