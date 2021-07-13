use crate::constants;
use crate::utils::bitutils;
use super::pieces::IndexPiece;

fn get_positive_attacks(occ: u32, direction: usize, square: usize) -> u32 {
    let mut attacks: u32 = constants::RAY_ATTACKS[direction][square];
    let blockers: u32 = attacks & occ;
    if blockers != 0 {
        let mut square = bitutils::bit_scan_forward(blockers as u64) as usize;
        square -= square / 5;
        attacks ^= constants::RAY_ATTACKS[direction][square];
    }
    attacks
}

fn get_negative_attacks(occ: u32, direction: usize, square: usize) -> u32 {
    let mut attacks: u32 = constants::RAY_ATTACKS[direction][square];
    let blockers: u32 = attacks & occ;
    if blockers != 0 {
        let mut square = bitutils::bit_scan_reverse(blockers as u64) as usize;
        square -= square / 5;
        attacks ^= constants::RAY_ATTACKS[direction][square];
    }
    attacks
}

fn get_single_ray_attacks(occ: u32, direction: usize, square: usize) -> u32 {
    match direction {
        7 | 0 | 1 | 2 => get_positive_attacks(occ, direction, square),
        3 | 4 | 5 | 6 => get_negative_attacks(occ, direction, square),
        _ => panic!("{} is not a valid direction", direction)
    }
}

pub fn get_slide_attacks(occ: u32, piece: &IndexPiece) -> u32 {
    match piece {
        IndexPiece::Rook(sq) => get_single_ray_attacks(occ, 0, *sq) |
                            get_single_ray_attacks(occ, 2, *sq) |
                            get_single_ray_attacks(occ, 4, *sq) |
                            get_single_ray_attacks(occ, 6, *sq),
        IndexPiece::Bishop(sq) => get_single_ray_attacks(occ, 1, *sq) |
                            get_single_ray_attacks(occ, 3, *sq) |
                            get_single_ray_attacks(occ, 5, *sq) |
                            get_single_ray_attacks(occ, 7, *sq)
    }
}
