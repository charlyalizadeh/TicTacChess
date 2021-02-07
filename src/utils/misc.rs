use nalgebra::Point2;

pub fn split_str_chunks<'a>(string: &'a str, chunk_size: usize) -> Vec<&'a str> {
    let nb_split = string.len() / chunk_size;
    let mut split_string: Vec<&str> = Vec::new();
    for i in (0..(nb_split * chunk_size)).step_by(chunk_size) {
        split_string.push(&string[i..i + chunk_size]);
    }
    return split_string;
}

pub fn set_board(board: &mut u32, coord: Point2<u8>) {
    // We don't need to take into account the original value in board for the game because
    // a given board/piece can only have on bit set to 1 at the same time
    *board = 1 << coord.y * 5 + coord.x;
}

pub fn reverse_string(string: &str) -> String {
    return string.chars().rev().collect::<String>();
}
