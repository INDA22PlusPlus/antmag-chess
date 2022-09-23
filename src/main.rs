

fn main() {
    
}

#[cfg(test)]
mod test{

    use test_crate::chess_api::*;
    use test_crate::chess_api::Board::*;
    use test_crate::chess_api::Util::*;
    use test_crate::chess_api::Queen::*;
    use test_crate::chess_api::Move_util::*;
    use test_crate::chess_api::Testing_interface::*;

    #[test]
    fn test_init_config(){
        let b : Board_state = create_init_board();
        
        let s = Cell::King; 
        println!("test!");
        ()
    }

    #[test]
    fn test_threat_generation_1(){
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_init_board();
        b._board_types[2][5] = Cell::Knight;
        b._board_types[7][6] = Cell::None;
        b._board_color[2][5] = false;
        update_threat_buffer(&mut b);

        assert_eq!(scheck(&b, true), true);
    }

    #[test]
    fn scheck_test_2(){
        let mut b : Board_state = create_init_board();
        update_threat_buffer(&mut b);
        assert_eq!(scheck(&b, true), false);
    }

    #[test]
    fn scheck_test_3(){
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_blank_board();
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
        let mut b : Board_state = create_blank_board();
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

    #[test]
    fn actual_actual_move_test_1(){
        let mut b : Board_state = create_blank_board();
        b._board_types[1][4] = Cell::King;
        b._board_types[3][4] = Cell::Knight;
        b._board_color[1][4] = BLACK;
        b._board_color[3][4] = WHITE;
        update_threat_buffer(&mut b);

        let mut move_buffer = construct_move_buffer();
        assert_eq!(get_move_list(&mut b, (1,4), &mut move_buffer), 6);
    }

    #[test]
    fn actual_actual_move_test_2(){
        let mut b : Board_state = create_blank_board();
        b._board_types[1][4] = Cell::King;
        b._board_types[3][4] = Cell::Knight;
        b._board_color[1][4] = BLACK;
        b._board_color[3][4] = WHITE;
        update_threat_buffer(&mut b);

        let test_move = Move{
            from : (3,4),
            to : (1,4),
            typ : Move_type::Capture,
            color : WHITE,
            promo_type : Cell::None
        };
        let test_move_2 = Move{
            from : (1,4),
            to : (1,5),
            typ : Move_type::Move,
            color : BLACK,
            promo_type : Cell::None
        };
        let test_move_3 = Move{
            from : (1,4),
            to : (1, 2),
            typ : Move_type::Move,
            color : BLACK,
            promo_type : Cell::None
        };

        assert_eq!(is_valid_move(&mut b, &test_move), false);
        assert_eq!(is_valid_move(&mut b, &test_move_2), false);
        assert_eq!(is_valid_move(&mut b, &test_move_3), false);
    }

    #[test]
    pub fn actual_actual_move_test_3(){
        let mut b : Board_state = create_blank_board();
        b._board_types[1][4] = Cell::Pawn;
        b._board_color[1][4] = BLACK;
        b._board_hasmoved[1][4] = false;

        b._board_types[2][3] = Cell::Pawn;
        b._board_color[2][3] = WHITE;
        b._board_hasmoved[2][3] = true;

        b._board_types[3][4] = Cell::Pawn;
        b._board_color[3][4] = WHITE;
        b._board_hasmoved[3][4] = false;

        b._board_types[2][5] = Cell::Pawn;
        b._board_color[2][2] = WHITE;
        b._board_hasmoved[2][5] = false;
        update_threat_buffer(&mut b);

        let mut white_threaten_cnt = 0;
        for i in 0..8{
            for j in 0..8{
                if(b.threat_buff[WHITE as usize][i][j] == true) {white_threaten_cnt += 1;}
            }
        }
        
        let mut move_buffer = construct_move_buffer();
        
        assert_eq!(get_move_list(&mut b, (2,5), &mut move_buffer), 3);
        assert_eq!(get_move_list(&mut b, (1,4), &mut move_buffer), 3);
        assert_eq!(get_move_list(&mut b, (2,3), &mut move_buffer), 2);
        assert_eq!(get_move_list(&mut b, (3,4), &mut move_buffer), 1);
        assert_eq!(white_threaten_cnt, 5);
    }

    #[test]
    pub fn sheck_mate_test_1(){
        let mut b = create_init_board();

        let b_pawn_mv_1 = Move{
            from : (1,1),
            to : (3, 1),
            typ : Move_type::Move,
            color : BLACK,
            promo_type : Cell::None
        };

        let b_pawn_mv_2 = Move{
            from : (1,2),
            to : (3, 2),
            typ : Move_type::Move,
            color : BLACK,
            promo_type : Cell::None
        };

        let w_pawn_mv_1 = Move{
            from : (6, 3),
            to : (2, 3),
            typ : Move_type::Move,
            color : WHITE,
            promo_type : Cell::None
        };

        let w_queen_mv = Move{
            from : (7, 3),
            to : (1, 2),
            typ : Move_type::Move,
            color : WHITE,
            promo_type : Cell::None
        };

        let b_king_mv = Move{
            from : (0, 4),
            to : (0,3),
            typ : Move_type::Capture,
            color : BLACK,
            promo_type : Cell::None
        };

        assert_eq!(is_valid_move(&mut b, &b_pawn_mv_1), true);
        assert_eq!(is_valid_move(&mut b, &b_pawn_mv_2), true);
        assert_eq!(is_valid_move(&mut b, &w_pawn_mv_1), false);
        assert_eq!(is_valid_move(&mut b, &w_queen_mv), false);
        assert_eq!(is_valid_move(&mut b, &b_king_mv), false);

        //Ignoring the turn-turn ordering
        make_move(&mut b, &b_pawn_mv_1);
        make_move(&mut b, &b_pawn_mv_2);
        make_move(&mut b, &w_pawn_mv_1);
        make_move(&mut b, &w_queen_mv); //This move is invalid
        make_move(&mut b, &b_king_mv);

        b._board_types[0][4] = Cell::Queen;
        b._board_color[0][4] = true;
        update_threat_buffer(&mut b);
        
        assert_eq!(is_scheck_mate(&mut b, BLACK), true);
        assert_eq!(is_scheck_mate(&mut b, WHITE), false);
    }
}