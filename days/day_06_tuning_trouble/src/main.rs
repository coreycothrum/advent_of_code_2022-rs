use std::collections::HashSet;
use std::fs;

fn find_read_index_with_X_unique_chars(input: String, unique_count: usize) -> Option<usize> {
    for (idx, window) in input.as_bytes().windows(unique_count).enumerate() {
        let mut unique = HashSet::<u8>::from_iter(window.iter().cloned());

        if unique.len() == unique_count {
            return Some(idx + unique_count);
        }
    }
    return None
}

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    let part1 = find_read_index_with_X_unique_chars(input, 4).unwrap();
    println!("{}", part1);
}
