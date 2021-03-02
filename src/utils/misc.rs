pub fn split_str_chunks<'a>(string: &'a str, chunk_size: usize) -> Vec<String> {
    let nb_split = string.len() / chunk_size;
    let mut split_string: Vec<String> = Vec::new();
    for i in (0..(nb_split * chunk_size)).step_by(chunk_size) {
        split_string.push(string[i..i + chunk_size].to_owned());
    }
    return split_string;
}

pub fn set_board(board: &mut u32, index: u32) {
    // We don't need to take into account the original value in board for the game because
    // a given board/piece can only have on bit set to 1 at the same time
    // We add `index / 4` because of the "hidden" column.
    *board = 1 << index + index / 4;
}

pub fn reverse_string(string: &str) -> String {
    return string.chars().rev().collect::<String>();
}
