// Ressources used for the following functions:
//     * https://www.chessprogramming.org/On_an_empty_Board
//     * https://www.chessprogramming.org/General_Setwise_Operations
pub fn remove_msb(nb: u32, shift: u8) -> u32 {
    (nb << shift) >> shift
}
pub fn remove_lsb(nb: u32, shift: u8) -> u32 {
    (nb >> shift) << shift
}
pub fn right_one(nb: u32) -> u32 {
    nb << 1 & 1014750
}
pub fn left_one(nb: u32) -> u32 {
    nb >> 1 & 507375
}

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
            cast = remove_msb(cast, shift);
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
        cast = right_one(cast);
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
            ne = remove_msb(ne, 12);
        }
        cast = right_one(cast);
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
            ne = remove_msb(ne, 12);
        }
        cast = left_one(cast);
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
            cast = remove_lsb(cast, 4);
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
            ne = remove_msb(ne, 12);
        }
        cast = left_one(cast);
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
            ne = remove_msb(ne, 12);
        }
        cast = right_one(cast);
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
            ne = remove_msb(ne, 12);
        }
        cast = left_one(cast);
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
