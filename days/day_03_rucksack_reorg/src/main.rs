use std::collections::HashSet;
use std::fs;

fn main() {
    let sum: u64 = fs::read_to_string("data/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let end: usize = l.len();
            let half: usize = end / 2;

            let comp1 = HashSet::<_>::from_iter(l[0..half].chars());
            let comp2 = HashSet::<_>::from_iter(l[half..end].chars());

            comp1.intersection(&comp2).fold(0, |acc, n| {
                // get ascii value and scale
                acc + (if n.is_lowercase() {
                    (*n as u8) - 96
                } else {
                    (*n as u8) - 38
                } as u64)
            })
        })
        .sum();
    println!("{sum}");
}
