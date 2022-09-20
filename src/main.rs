

fn main() {
    
}

#[cfg(test)]
mod test{

    use test_crate::chess_api::*;
    use test_crate::chess_api::Board::*;
    use test_crate::chess_api::util::*;
    use test_crate::chess_api::Queen::*;

    use crate::test;


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

    #[test]
    fn move_test_1(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[1][5] = Cell::Pawn;
        b._board_color[0][5] = false;
        let li = [
            (0,5),
        ];

        let mut move_buffer:[Move; MAX_MOVES] = construct_move_buffer();

        let cnt = generate_moves_simple_static((1,5), &li, &mut b, &mut move_buffer);
        assert_eq!(cnt, 1);
        if let Move_type::Promotion = move_buffer[0].typ {} else {panic!()}
    }

    #[test]
    fn move_test_2(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[3][5] = Cell::King;
        b._board_color[0][5] = true;

        let li = [
            (2,5),
            (3,4),
            (3,6)
        ];
        let mut move_buffer:[Move; MAX_MOVES] = construct_move_buffer();
        let cnt = generate_moves_simple_static((3,5), &li, &mut b, &mut move_buffer);
        assert_eq!(cnt, 3);
        
        for i in 0..3 {
            if let Move_type::Move = move_buffer[i].typ {} else {panic!();}
        }
    }

    #[test]
    fn move_test_3(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[3][5] = Cell::Knight;
        b._board_types[1][4] = Cell::Pawn;
        b._board_color[3][5] = false;
        b._board_color[1][4] = true;
        let mut move_buffer = construct_move_buffer();
        
        let cnt = generate_moves_simple_static((3,5), &[(1,4)], &mut b, &mut move_buffer);
        assert_eq!(cnt, 1);
        if let Move_type::Capture = move_buffer[0].typ {} else {panic!();}
    }

    #[test]
    fn actual_move_test_1(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[4][4] = Cell::Queen;
        b._board_types[3][4] = Cell::Pawn;
        b._board_types[4][1] = Cell::Pawn;

        b._board_color[4][4] = BLACK;
        b._board_color[3][4] = BLACK;
        b._board_color[4][1] = WHITE;

        let mut move_buffer = construct_move_buffer();

        let cnt = Queen::generate_moves_simple(4, 4, &mut b, &mut move_buffer);
        assert_eq!(cnt, 22);
    }

    #[test]
    fn actual_move_test_2(){
        let mut b : Board_state = create_blank_board(false);
        b._board_types[0][7] = Cell::King;
        b._board_types[3][4] = Cell::Knight;
        b._board_color[0][7] = BLACK;
        b._board_color[3][4] = WHITE;

        let mut move_buffer_1 = construct_move_buffer();
        let cnt_1 = King::generate_moves_simple(0, 7, &mut b, &mut move_buffer_1);

        let mut move_buffer_2 = construct_move_buffer();
        let cnt_2 = Knight::generate_moves_simple(3, 4, &mut b, &mut move_buffer_2);

        assert_eq!(cnt_1, 3);
        assert_eq!(cnt_2, 8);
    }


}