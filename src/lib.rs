
/*




board structure: Use array, [[]]
enum to represent an arbitrary piece or empty; 
Utilize a simple threat_buffer that is recomputed every move

*/

#[allow(unused_parens)]
#[allow(dead_code)]
pub mod chess_api{
    use std::slice;
    
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

    pub mod Board{
        use std::mem::swap;
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
        use super::*;

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
    }

    pub mod Knight{
        use super::Board_state;
        use super::util::generate_threat_static;

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
        /* 
        pub fn generate() -> u64{
            
        }
        */
    }

    pub mod Bishop{
        use super::Board_state;
        use super::util::generate_threat_dir;

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
    }

    pub mod Rook{
        use super::Board_state;
        use super::util::generate_threat_dir;

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
    }

    pub mod Queen{
        use super::Board_state;
        use super::util::generate_threat_dir;

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
    }

    pub mod King{
        use super::Board_state;
        use super::util::generate_threat_static;

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
    }

    pub mod Pawn{
        use super::Board_state;
        use super::util::generate_threat_static;
        
        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let mut row_increment = if color == false {-1} else {1}; //if color is white
            if(board.inverted == true) { //The board could be inverted
                if(row_increment == 1) { // asuming row increment is either 1 or -1
                    row_increment = -1;
                }else{
                    row_increment = 1;
                }
            } 
            
            let li : [(i32,i32); 2] = [
                (row + row_increment, col + 1),
                (row + row_increment, col - 1)
            ];
            generate_threat_static(&li, board, color);
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




