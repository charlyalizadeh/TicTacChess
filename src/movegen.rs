
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
        
        // Horizontal mask
        let right_border = 8 << (5 * row);
        let left_border = 1 << (5 * row);
        let mut shift = 1;
        // TODO: Combine those two loops ? maybe a good exercise to look the generated ASM
        while board << shift < right_border {
            final_mask |= board << shift;
            shift += 1;
        }
        shift = 1;
        while board >> shift > left_border {
            final_mask |= board >> shift;
            shift += 1;
        }

        // Vertical mask
        shift = 5;
        while (board << shift) / 32768 == 0 {
            final_mask |= board << shift;
            shift += 5;
        }
        shift = 5;
        while (board >> shift) / 16 != 0 {
            final_mask |= board >> shift;
            shift += 5;
        }
        final_mask
    }
}
