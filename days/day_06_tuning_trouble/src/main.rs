use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    for (idx, window) in input.as_bytes().windows(4).enumerate() {
        let mut unique = HashSet::<u8>::from_iter(window.iter().cloned());

        if unique.len() == 4 {
            println!("{:?}", unique);
            println!("{}", idx + 4);
            break;
        }
    }
}
