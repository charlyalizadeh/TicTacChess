use crate::constants::RAY_ATTACKS;
use crate::utils::bitscan;
use crate::find_magic::pieces::IndexPiece;


pub fn gen_blocker_boards(piece: &IndexPiece) -> Vec<u32> {
    let mask = match *piece {
        IndexPiece::Rook(sq) => RAY_ATTACKS[0][sq] |
                                RAY_ATTACKS[2][sq] |
                                RAY_ATTACKS[4][sq] |
                                RAY_ATTACKS[6][sq],
        IndexPiece::Bishop(sq) => RAY_ATTACKS[1][sq] |
                                RAY_ATTACKS[3][sq] |
                                RAY_ATTACKS[5][sq] |
                                RAY_ATTACKS[7][sq]
    };
    gen_blocker_boards_from_mask(mask)
}

fn gen_blocker_boards_from_mask(mask: u32) -> Vec<u32> {
    let nb_set_bits = mask.count_ones();
    let set_bits_index = get_set_bits_index(mask);
    let all_combination:Vec<u32> = (0..(u32::pow(2, nb_set_bits))).map(u32::from).collect();
    let all_board = all_combination.into_iter().map(|bits| map_bits_to_board(bits, &set_bits_index)).collect();
    all_board
}

fn map_bits_to_board(bits: u32, index: &[u32]) -> u32 {
    let mut board: u32 = 0;
    for (i, shift) in index.iter().enumerate() {
        if (1 << i & bits) != 0 {
            board = board | 1 << shift;
        }
    }
    board
}

fn get_set_bits_index(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut bits_index = Vec::new();
    while n != 0 {
        let index = bitscan::bit_scan_forward(n as u64);
        bits_index.push(index as u32);
        n ^= 1 << index;
    }
    bits_index
}
