
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
        _board_hasmoved : [[bool; 8]; 8],

        threat_buff : [[bool; 8]; 8], //is king in scheck
        turn : bool, //0 is white, 1 is black
        castling: [bool; 2],
        
        king_pos : [(u8, u8); 2],
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

    pub fn create_init_board(inverted : bool) -> Board_state{
        let mut B = Board_state{
            _board_types : [[Cell::None; 8]; 8],
            _board_hasmoved : [[false; 8]; 8],

            threat_buff : [[false; 8]; 8],
            turn : false,
            castling : [false; 2],
            king_pos : [(0,0); 2]
        };
        create_start_config(&mut B, inverted);
        return B;
    }

    pub fn create_start_config(board : &mut Board_state, inverted : bool){
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
        }
        row += row_dir;
        for i in 0..8{
            board._board_types[row as usize][i] = Cell::Pawn;
        }
    }

    pub fn scheck(){
        
    }
    
    pub mod util{
        use std::i32;
        use super::*;

        pub fn generate_threat_static(li : &[(i32, i32)], board : &Board_state, buff : &mut [[bool; 8]; 8]){
            let t = li[0];
            for (row, col) in li.iter() {
                if(row < &0 || row > &7 || col < &0 || col > &7){ continue;}
                
                let at : &Cell = &board._board_types[*row as usize][*col as usize];
                if let Cell::None = at {
                    buff[*row as usize][*col as usize] = true;
                }
            }
        }
    }

    pub mod Knight{
        use super::Board_state;
        use super::util;

        pub fn generate_simple(mut buff: &[u64]) -> u64{
            
        }

        pub fn generate_threat(row : i32, col : i32, board : &Board_state, buff : &mut [[bool; 8]; 8]){
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

            


        }

        pub fn generate() -> u64{
            
        }
    }

    pub mod King{
        
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




