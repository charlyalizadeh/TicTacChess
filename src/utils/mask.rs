use crate::utils::bitutils;
// Positive rays
pub fn gen_raycast_top(mask_border: bool) -> [u32;16] {
    let mut cast = 33824;
    let mut limit = 1048575;
    let mut shift = 12;
    if mask_border {
        cast = 1056;
        limit = 32768;
        shift = 17;
    }
    let mut raycast: [u32;16] = [0;16];
    let mut j = 0;
    for i in 0..20 {
        if i % 5 != 4 {
            raycast[j] = cast;
            j += 1;
        }
        cast <<= 1;
        // If some bits "above" the board are set we want to remove them.
        if cast > limit  {
            cast = bitutils::remove_msb(cast, shift);
        }
    }
    raycast
}
pub fn gen_raycast_right(mask_border: bool) -> [u32;16] {
    let mut cast: u32 = 14;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 236775 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[f + 4 * (i / 5)] = ne & limit;
            ne <<= 5;
            //ne = remove_msb(ne, 12);
        }
        cast = bitutils::right_one(cast);
    }
    raycast
}
pub fn gen_raycast_topright(mask_border: bool) -> [u32;16] {
    let mut cast: u32 = 266304;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 236775 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[f + 4 * (i / 5)] = ne & limit;
            ne <<= 5;
            ne = bitutils::remove_msb(ne, 12);
        }
        cast = bitutils::right_one(cast);
    }
    raycast
}
pub fn gen_raycast_topleft(mask_border: bool) -> [u32;16] {
    let mut cast = 34944;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 473550 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[(3 - f) + 4 * (i / 5)] = ne & limit;
            ne <<= 5;
            ne = bitutils::remove_msb(ne, 12);
        }
        cast = bitutils::left_one(cast);
    }
    raycast
}

// Negative rays
pub fn gen_raycast_bottom(mask_border: bool) -> [u32;16] {
    let mut cast = 8456;
    if mask_border {
        cast = 8448;
    }
    let mut raycast: [u32;16] = [0;16];
    let mut j = 0;
    for i in 0..20 {
        if i % 5 != 4 {
            raycast[15 - j] = cast;
            j += 1;
        }
        cast >>= 1;
        if mask_border && [4, 8].contains(&j) {
            cast = bitutils::remove_lsb(cast, 4);
        }
    }
    raycast
}
pub fn gen_raycast_left(mask_border: bool) -> [u32;16] {
    let mut cast: u32 = 229376;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 473550 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[(3 - f) + 4 * (3 - (i / 5))] = ne & limit;
            ne >>= 5;
            ne = bitutils::remove_msb(ne, 12);
        }
        cast = bitutils::left_one(cast);
    }
    raycast
}
pub fn gen_raycast_bottomright(mask_border: bool) -> [u32;16] {
    let mut cast: u32 = 2184;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 6336 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[f + 4 * (3 - (i / 5))] = ne & limit;
            ne >>= 5;
            ne = bitutils::remove_msb(ne, 12);
        }
        cast = bitutils::right_one(cast);
    }
    raycast
}
pub fn gen_raycast_bottomleft(mask_border: bool) -> [u32;16] {
    let mut cast: u32 = 4161;
    let mut raycast: [u32;16] = [0;16];
    let limit: u32 = if mask_border { 6336 } else { 507375 };
    for f in 0..4 {
        let mut ne: u32 = cast;
        for i in (0..20).step_by(5) {
            if i % 5 == 4 {
                continue;
            }
            raycast[(3 - f) + 4 * (3 - (i / 5))] = ne & limit;
            ne >>= 5;
            ne = bitutils::remove_msb(ne, 12);
        }
        cast = bitutils::left_one(cast);
    }
    raycast
}
pub fn gen_ray_attacks(mask_border: bool) -> [[u32;16];8] {
    [
        gen_raycast_top(mask_border),
        gen_raycast_topright(mask_border),
        gen_raycast_right(mask_border),
        gen_raycast_bottomright(mask_border),
        gen_raycast_bottom(mask_border),
        gen_raycast_bottomleft(mask_border),
        gen_raycast_left(mask_border),
        gen_raycast_topleft(mask_border),
    ]
}

//        noNoWe    noNoEa
//            +9    +11
//             |     |
//noWeWe   +3__|     |__+7  noEaEa
//              \   /
//               >0<
//           __ /   \ __
//soWeWe  -7   |     |  -3  soEaEa
//             |     |
//            -11   -9
//        soSoWe    soSoEa
fn gen_knight_mask(sq: usize) -> u32 {
    let index: u32 = 1 << (sq + sq / 4);
    let mask =  (index << 11 & 473088) |
                (index << 7 & 405900) |
                (index >> 3 & 12684) |
                (index >> 9 & 462) |
                (index >> 11 & 231) |
                (index >> 7 & 3171) |
                (index << 3 & 101472) |
                (index << 9 & 236544);
    mask
}
pub fn gen_knight_masks() -> [u32;16] {
    let mut masks: [u32;16] = [0;16];
    for i in 0..16 {
        masks[i] = gen_knight_mask(i);
    }
    masks
}

fn gen_pawn_attack_top(sq: usize) -> u32 {
    let sq = sq + sq / 4;
    match sq {
       15..=18 => 0, // Top row
       _ => (1 << (sq + 4) | 1 << (sq + 6)) & 507375
    }
}
fn gen_pawn_attack_bottom(sq: usize) -> u32 {
    let sq = sq + sq / 4;
    match sq {
       0..=3 => 0, // Bottom row
       4..=5 => (1 << (sq - 4)) & 507375, // Can't have sq - 6 when sq < 4
       _ => (1 << (sq - 4) | 1 << (sq - 6)) & 507375
    }
}
pub fn gen_pawn_masks_top() -> [u32;16] {
    let mut masks: [u32;16] = [0;16];
    for i in 0..12 {
        masks[i] = bitutils::top_one(1 << (i + i / 4));
    }
    masks
}
pub fn gen_pawn_masks_bottom() -> [u32;16] {
    let mut masks: [u32;16] = [0;16];
    for i in 4..16 {
        masks[i] = bitutils::bottom_one(1 << (i + i / 4));
    }
    masks
}
pub fn gen_pawn_attacks_top() -> [u32;16] {
    let mut masks: [u32;16] = [0;16];
    for i in 0..16 {
        masks[i] = gen_pawn_attack_top(i);
    }
    masks
}
pub fn gen_pawn_attacks_bottom() -> [u32;16] {
    let mut masks: [u32;16] = [0;16];
    for i in 0..16 {
        masks[i] = gen_pawn_attack_bottom(i);
    }
    masks
}
