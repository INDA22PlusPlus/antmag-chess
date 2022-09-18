
/*



board structure: Use array, [[]]
enum to represent an arbitrary piece or empty; 
Utilize a simple threat_buffer that is recomputed every move

*/

#[allow(unused_parens)]


pub mod chess_api{
    use std::slice;
    
    struct Board_state{
        //state defining parameeters
        board : [[Cell; 64]; 64],
        threat_buff : [u64; 2], //is king in scheck
        turn : bool, //0 is white, 1 is black
        castling: [bool; 2],
        
        king_pos : [(u8, u8); 2],
    }

    struct Piece_struct{
        has_moved : bool,
    }

    enum Cell{
        King(Piece_struct),
        Queen(Piece_struct),
        Bishop(Piece_struct),
        Rook(Piece_struct),
        Knight(Piece_struct),
        Pawn(Piece_struct),
        None
    }

    fn create_start_config_color(color : bool, inverted : bool){
        let start_row = if(color as i32 == 0 && !inverted) {7} else {0};
        let row_dir = if(color as i32 == 0 && !inverted) {-1} else {1};
        let init_piece_struct = Piece_struct{
            has_moved : false,
        };

        let row1_layout = [
            Cell::Rook(init_piece_struct),
            Cell::Knight(init_piece_struct),
            Cell::Bishop(init_piece_struct),
            Cell::Queen(init_piece_struct),
            Cell::King(init_piece_struct),
            Cell::Bishop(init_piece_struct),
            Cell::Knight(init_piece_struct),
            Cell::Rook(init_piece_struct)
        ];

        for i in 0..8{

        }


    }

    pub fn scheck(){

    }

    pub mod Knight{
        pub fn generate_simple(mut buff: &[u64]) -> u64{
            
        }

        pub fn generate() -> u64{
            
        }
    }

    pub mod King{
        
    }



}
