
/*




board structure: Use array, [[]]
enum to represent an arbitrary piece or empty; 
Utilize a simple threat_buffer that is recomputed every move

todo: Move generation
    - Representing moves
todo: Scheck mate


Implament a generate_moves_simple for each piece type
implament a move_check function that determines if the move is valid(simulate move and recompute threat buffer)

*/

#[allow(unused_parens)]
#[allow(dead_code)]
pub mod chess_api{
    use std::slice;
    
    pub const WHITE : bool = false;
    pub const BLACK : bool = true;
    pub const MAX_MOVES : usize = 32;

    #[derive(Clone, Copy)]
    pub struct Board_state{
        //state defining parameeters
        pub _board_types : [[Cell; 8]; 8],
        pub _board_hasmoved : [[bool; 8]; 8],
        pub _board_color : [[bool; 8]; 8],

        pub threat_buff : [[[bool; 8]; 8]; 2], //is king in scheck, only public for testing
        turn : bool, //0 is white, 1 is black
        inverted : bool,
        castling: [bool; 2],
        
        pub king_pos : [(usize, usize); 2],
    }

    #[derive(Clone, Copy)]
    pub enum Cell{
        King,
        Queen,
        Bishop,
        Rook,
        Knight,
        Pawn,
        None
    }

    #[derive(Clone, Copy)]
    pub enum Move_type{
        Peassant,
        Castling,
        Capture,
        Promotion,
        Move
    }

    #[derive(Clone, Copy)]
    pub struct Move{
        pub from : (i32, i32),
        pub to : (i32, i32),
        pub typ : Move_type,
        pub color : bool,
    }

    pub mod Board{
        use std::{mem::swap, option};
        use super::*;

        //Essentially a board constructor
        pub fn create_init_board(inverted : bool) -> Board_state{
            let mut B = Board_state{
                _board_types : [[Cell::None; 8]; 8],
                _board_hasmoved : [[false; 8]; 8],
                _board_color : [[false; 8]; 8],

                threat_buff : [[[false; 8]; 8]; 2],
                turn : false,
                inverted : inverted,
                castling : [false; 2],
                king_pos : [(0,0); 2]
            };

            B.king_pos[0] = (7,4);
            B.king_pos[1] = (0,4);
            if(inverted) {B.king_pos[0] = (0,4); B.king_pos[1] = (7,4);}
            // Why can't i do swap(&mut B.king_pos[0], &mut B.king_pos[1]) ? I guess B.king_pos becomes mutable in both?

            create_start_config(&mut B, inverted);
            update_threat_buffer(&mut B);

            return B;
        }

        pub fn create_blank_board(inverted : bool) -> Board_state {
            let mut B = Board_state{
                _board_types : [[Cell::None; 8]; 8],
                _board_hasmoved : [[false; 8]; 8],
                _board_color : [[false; 8]; 8],

                threat_buff : [[[false; 8]; 8]; 2],
                turn : false,
                inverted : inverted,
                castling : [false; 2],
                king_pos : [(0,0); 2]
            };

            return B;
        }

        /*Moves can be created if the following conditions are met: 
        (1) there is a piece on from 
        (2) If there is a piece at to, it is not of the same color
        */
        pub fn get_move_type(from : (i32, i32), to : (i32,i32), board : &Board_state) -> Option<Move_type>{
            let to_type = board._board_types[to.0 as usize][to.1 as usize];
            let from_type = board._board_types[from.0 as usize][from.1 as usize];
            if let Cell::None = from_type {return None;}

            let from_color = board._board_color[from.0 as usize][from.1 as usize]; //The color making a move
            if let Cell::None = to_type{ //Promotion or Move
                if let Cell::Pawn = from_type{
                    if((to.0 == 0 && from_color == WHITE) || (to.0 == 7 && from_color == BLACK)){
                        return Some(Move_type::Promotion);
                    }else{
                        return Some(Move_type::Move);
                    }
                }else{
                    return Some(Move_type::Move);
                }
            }else{
                if board._board_color[to.0 as usize][to.1 as usize] == from_color{
                    //Check for castling here
                    return None;
                }else{ //Capture
                    return Some(Move_type::Capture);
                }
            }; 
        }

        pub fn create_move(from : (i32, i32), to : (i32,i32), board : &Board_state) -> Option<Move> {
            let typ = get_move_type(from, to, board);
            
            match typ{
                Some(p) => {
                    return Some(Move{
                        from : from,
                        to : to,
                        typ : p,
                        color : board._board_color[from.0 as usize][from.1 as usize]
                    });
                },
                None => {return None;}
            };
        }

        fn create_start_config(board : &mut Board_state, inverted : bool){
            create_start_config_color(board, false, inverted);
            create_start_config_color(board, true, inverted);
        }

        fn create_start_config_color(board : &mut Board_state, color : bool, inverted : bool){
            let mut row:i32 = if(color as i32 == 0 && !inverted) {7} else {0};
            let row_dir = if(color as i32 == 0 && !inverted) {-1} else {1};

            let row1_layout = [
                Cell::Rook,
                Cell::Knight,
                Cell::Bishop,
                Cell::Queen,
                Cell::King,
                Cell::Bishop,
                Cell::Knight,
                Cell::Rook
            ];
    
            for i in 0..8{
                board._board_types[row as usize][i] = row1_layout[i];
                board._board_color[row as usize][i] = color;
            }
            row += row_dir;
            for i in 0..8{
                board._board_types[row as usize][i] = Cell::Pawn;
                board._board_color[row as usize][i] = color;
            }
        }

        pub fn update_threat_buffer(board : &mut Board_state){
            for row in 0..8 {
                for col in 0..8 {
                    let at = &board._board_types[row][col]; //Having implamented the copy trait
                    let at_color = board._board_color[row][col];
                    
                    match at{
                        Cell::Pawn => Pawn::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Knight => Knight::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Bishop => Bishop::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Rook => Rook::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Queen => Queen::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::King => King::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::None => continue
                    };
                }
            }
        }
    }

    pub fn scheck(board : &Board_state, color : bool) -> bool { //is king of color(color) in scheck?
        let (x,y) = board.king_pos[color as usize];
        return board.threat_buff[(!color) as usize][x][y];
    }
    
    pub mod util{
        use std::i32;
        use super::{*, Board::create_move};

        pub fn move_check(board : &Board_state, mv : &Move) -> bool{

            //If we are moving a king
            if let Cell::King = board._board_types[mv.from.0 as usize][mv.from.1 as usize] {
                //Check if the color not doing the move is threatening the cell that we want to move the king to
                if(board.threat_buff[(!mv.color) as usize][mv.to.0 as usize][mv.to.1 as usize] == true) {return false;}
            }
           
            //We should not be able to make a move that will capture a king
            if let Cell::King = board._board_types[mv.to.0 as usize][mv.to.1 as usize]{return false;}

            //Make a copy of the board and simulate a move, update_threat_buffer, check if check.

            return true;
        }

        pub fn generate_threat_static(li : &[(i32, i32)], board : &mut Board_state, color : bool){
            for (row, col) in li.iter() {
                if(row < &0 || row > &7 || col < &0 || col > &7){ continue;}
                
                let at : &Cell = &board._board_types[*row as usize][*col as usize];
                board.threat_buff[color as usize][*row as usize][*col as usize] = true;
            }
        }

        pub fn generate_threat_dir(mut row : i32, mut col : i32, dir : (i32, i32), board : &mut Board_state, color : bool){
            row += dir.0;
            col += dir.1;

            while(row >= 0 && row < 8 && col >= 0 && col < 8){
                let at : &Cell = &board._board_types[row as usize][col as usize];
                board.threat_buff[color as usize][row as usize][col as usize] = true;
                if let Cell::None = at{}else{
                    break;
                }
                
                row += dir.0;
                col += dir.1;
            }
        }

        pub fn construct_move_buffer() -> [Move; MAX_MOVES]{
            return [Move {
                from : (0,0),
                to : (0,0),
                typ : Move_type::Move,
                color : false,
            }; MAX_MOVES];
        }

        //We generate the moves for a piece of color(color) given the positions listed in li
        pub fn generate_moves_simple_static(from : (i32,i32), li : &[(i32, i32)], board : &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let mut buffer_idx: usize= 0;
            assert!(buffer_idx < MAX_MOVES);

            for (row, col) in li{
                if(*row < 0 || *row > 7 || *col < 0 || *col > 7){ continue;}

                //If this errors then the move we tried to create was not remotely valid
                let res = create_move(from, (*row, *col), board);
                if let Option::Some(p) = res {
                    move_buffer[buffer_idx] = p;
                    buffer_idx += 1;
                }
            }

            return buffer_idx;
        }

        pub fn generate_moves_simple_dir(from : (i32,i32), dir : (i32, i32), board : &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let mut buffer_idx: usize= 0;
            assert!(buffer_idx < MAX_MOVES);

            let from_color = board._board_color[from.0 as usize][from.1 as usize];
            let mut to = (from.0 + dir.0, from.1 + dir.1);
            let mut break_flag : bool = false;
            
            while(to.0 >= 0 && to.0 < 8 && to.1 >= 0 && to.1 < 8 && !break_flag){
                let at = board._board_types[to.0 as usize][to.1 as usize];
                let at_color = board._board_color[to.0 as usize][to.1 as usize];
                if let Cell::None = at {} else{
                    break_flag = true;
                    if(from_color == at_color) {continue;}
                }

                move_buffer[buffer_idx] = create_move(from, to, board).unwrap(); //This is expected to work
                buffer_idx += 1;
                to.0 += dir.0;
                to.1 += dir.1;
            }

            return buffer_idx;
        }


    }

    pub mod Knight{
        use super::{Board_state, MAX_MOVES};
        use super::Move;
        use super::util::*;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let li : [(i32, i32); 8] = [
                (row+1, col-2),
                (row+2, col-1),
    
                (row+2, col+1),
                (row+1, col+2),
                
                (row-1, col+2),
                (row-2, col+1),
                
                (row-2, col-1),
                (row-1, col-2)
            ];
            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let li : [(i32, i32); 8] = [
                (row+1, col-2),
                (row+2, col-1),
    
                (row+2, col+1),
                (row+1, col+2),
                
                (row-1, col+2),
                (row-2, col+1),
                
                (row-2, col-1),
                (row-1, col-2)
            ];
            return generate_moves_simple_static((row, col), &li, board, move_buffer);
        }

        /* 
        pub fn generate() -> u64{
            
        }
        */
    }

    pub mod Bishop{
        use super::MAX_MOVES;
        use super::Board_state;
        use super::Move;
        use super::util::*;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32,i32); 4] = [
                (-1,-1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32,i32); 4] = [
                (-1,-1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            let mut curr_cnt = 0;

            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row,col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod Rook{
        use super::Board_state;
        use super::util::*;
        use super::Move;
        use super::MAX_MOVES;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32, i32); 4] = [
                (1,0),
                (-1,0),
                (0,1),
                (0,-1)
            ];

            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32, i32); 4] = [
                (1,0),
                (-1,0),
                (0,1),
                (0,-1)
            ];
            let mut curr_cnt = 0;
            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row, col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod Queen{
        use super::Board_state;
        use super::util::*;
        use super::MAX_MOVES;
        use super::Move;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32, i32); 8] = [
                (1, 0),
                (-1,0),
                (0,1),
                (0,-1),
                (-1, -1),
                (-1,1),
                (1,-1),
                (1,1)
            ];

            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32, i32); 8] = [
                (1, 0),
                (-1,0),
                (0,1),
                (0,-1),
                (-1, -1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            
            let mut curr_cnt = 0;
            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row, col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod King{
        use super::Board_state;
        use super::util::*;
        use super::MAX_MOVES;
        use super::Move;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let li : [(i32, i32); 8] = [
                (row+1, col+1),
                (row-1, col+1),
                (row+1, col-1),
                (row-1, col-1),
                
                (row-1, col),
                (row, col-1),
                (row+1, col),
                (row, col+1)
            ];

            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let li : [(i32, i32); 8] = [
                (row+1, col+1),
                (row-1, col+1),
                (row+1, col-1),
                (row-1, col-1),
                
                (row-1, col),
                (row, col-1),
                (row+1, col),
                (row, col+1)
            ];

            return generate_moves_simple_static((row,col), &li, board, move_buffer);
        }
    }

    pub mod Pawn{
        use super::util::*;
        use super::*;
        
        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let row_increment = if color == false {-1} else {1}; //if color is white
            
            let li : [(i32,i32); 2] = [
                (row + row_increment, col + 1),
                (row + row_increment, col - 1)
            ];
            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, color : bool, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let row_increment = if color == false {-1} else {1}; //if color is white
            let mut li : [(i32,i32); 3] = [
                (row + row_increment, col),
                (-1, -1), //Terrible workaround, these will be ignored by the static generator
                (-1, -1)
            ];
            
            if let Cell::None = board._board_types[(row + row_increment) as usize][(col-1) as usize] {} else {
                li[1] = (row + row_increment, col-1);
            }
            if let Cell::None = board._board_types[(row + row_increment) as usize][(col+1) as usize] {} else {
                li[2] = (row + row_increment, col+1);
            }
            return generate_moves_simple_static((row, col), &li, board, move_buffer);
        }

    }
}


/*
pub struct Board_state{
        //state defining parameeters
        _board : [[Cell; 8]; 8],
        threat_buff : [u64; 2], //is king in scheck
        turn : bool, //0 is white, 1 is black
        castling: [bool; 2],
        
        king_pos : [(u8, u8); 2],
    }

*/




