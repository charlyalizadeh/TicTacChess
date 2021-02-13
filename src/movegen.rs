
pub struct MoveGen;

// I didn't look the litterature for those functions, so there are a lot of chance that
// simpler/cleaner/faster solutions exists.
// Those functions only need to be run once at the beggining of a session so not a big deal.
// Also I'm not sure of the pertinence of creating a struct to regroup the function vs just
// defining multiple function outside a struct.
impl MoveGen {
    pub fn gen_rook_mask() -> [u32;16] {
        let mut rook_mask: [u32;16] = [0; 16];
        let mut index = 0;
        for i in 0..20 {
            if i % 5 == 4 {
                continue;
            }
            rook_mask[index] = MoveGen::gen_rook_mask_from_board(1 << i);
            index += 1;
        }
        rook_mask
    }
    fn gen_rook_mask_from_board(board: u32) -> u32 {
        let mut final_mask: u32 = 0;
        let mut row = 0;
        while board >> row * 5 + 5 > 0 {
            row += 1;
        }
        
        let right_border = 8 << (5 * row);
        let left_border = 1 << (5 * row);

        let mut shift = 5;
        // TODO: Combine those two loops ? maybe a good exercise to look the generated ASM
        // Top
        while (board << shift) / 32768 == 0 {
            final_mask |= board << shift;
            shift += 5;
        }
        // Bottom
        shift = 5;
        while (board >> shift) / 16 != 0 {
            final_mask |= board >> shift;
            shift += 5;
        }
        // Right
        shift = 1;
        while board << shift < right_border {
            final_mask |= board << shift;
            shift += 1;
        }
        // Left
        shift = 1;
        while board >> shift > left_border {
            final_mask |= board >> shift;
            shift += 1;
        }
        final_mask
    }
    pub fn gen_bishop_mask() -> [u32; 16] {
        let mut bishop_mask: [u32;16] = [0; 16];
        let mut index = 0;
        for i in 0..20 {
            if i % 5 == 4 {
                continue;
            }
            bishop_mask[index] = MoveGen::gen_bishop_mask_from_board(1 << i);
            index += 1;
        }
        bishop_mask
    }
    fn gen_bishop_mask_from_board(board: u32) -> u32 {
        let mut final_mask: u32 = 0;
        let top_right_borders: [u32;7] = [8, 256, 8192, 262144, 131072, 65536, 32768];
        let top_left_borders: [u32;7] = [1, 32, 1024, 32768, 65536, 131072, 262144];
        let bottom_right_borders: [u32;7] = [1, 2, 4, 8, 256, 8192, 262144];
        let bottom_left_borders: [u32;7] = [1, 2, 4, 8, 32, 1024, 32768];

        let mut shift = 6;
        // Top-Right
        while board < 8192 && !top_right_borders.iter().any(|x| [board, board << shift].contains(x)) {
            final_mask |= board << shift;
            shift += 6;
        }
        // Top-Left
        shift = 4;
        while board < 8192 && !top_left_borders.iter().any(|x| [board, board << shift].contains(x)) {
            final_mask |= board << shift;
            shift += 4;
        }
        // Bottom-Right
        shift = 4;
        while board > 8 && !bottom_right_borders.iter().any(|x| [board, board >> shift].contains(x)) {
            final_mask |= board >> shift;
            shift += 4;
        }
        // Bottom-Left
        shift = 6;
        println!("{}", board);
        while board > 8 && !bottom_left_borders.iter().any(|x| [board, board >> shift].contains(x)) {
            final_mask |= board >> shift;
            shift += 6;
        }
        final_mask
    }
}
