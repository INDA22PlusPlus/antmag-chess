

fn main() {
    
}

#[cfg(test)]
mod test{
    use test_crate::chess_api::*;
    use test_crate::chess_api::Board::*;

    #[test]
    fn test_init_config(){
        let b : Board_state = create_init_board(false);
        
        let s = Cell::King; 
        println!("test!");
        ()
    }

    #[test]
    fn test_threat_generation_1(){
        let mut b : Board_state = create_blank_board(false);
        let queen_placements = [
            (0,4),
            (1,1),
            (2,3),
            (3,6),
            (4,2),
            (5,7),
            (6,5),
            (7,0)
        ];
        let mut expected_result = [[true; 8]; 8];
        for (row, col) in queen_placements{
            b._board_types[row][col] = Cell::Queen;
            expected_result[row][col] = false;
        }
        update_threat_buffer(&mut b);

        assert_eq!(expected_result, b.threat_buff[0]);
    }

    #[test]
    fn sheck_test_1(){
        let mut b : Board_state = create_init_board(false);
        b._board_types[2][5] = Cell::Knight;
        b._board_types[7][6] = Cell::None;
        b._board_color[2][5] = false;
        update_threat_buffer(&mut b);

        assert_eq!(scheck(&b, true), true);
    }

    #[test]
    fn scheck_test_2(){
        let mut b : Board_state = create_init_board(false);
        update_threat_buffer(&mut b);
        assert_eq!(scheck(&b, true), false);
    }

    #[test]
    fn scheck_test_3(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[3][6] = Cell::Rook;
        b._board_color[3][6] = false; // white

        b._board_types[3][7] = Cell::King;
        b._board_color[3][7] = true; // black
        
        b._board_types[4][5] = Cell::King; 
        b._board_color[4][5] = false; //white

        b.king_pos[0] = (4,5);
        b.king_pos[1] = (3,7);

        update_threat_buffer(&mut b);
        assert_eq!(scheck(&b, true), true);
    }



}