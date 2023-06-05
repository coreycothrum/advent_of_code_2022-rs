use std::collections::HashSet;
use std::fs;

fn priority_mapping(c: &char) -> u8 {
    // map to ascii, then scale
    if c.is_lowercase() {
        (*c as u8) - 96
    } else {
        (*c as u8) - 38
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    let sum: u64 = input
        .lines()
        .map(|l| {
            let end: usize = l.len();
            let half: usize = end / 2;

            let comp1 = HashSet::<_>::from_iter(l[0..half].chars());
            let comp2 = HashSet::<_>::from_iter(l[half..end].chars());

            comp1
                .intersection(&comp2)
                .fold(0, |acc, c| acc + (priority_mapping(c) as u64))
        })
        .sum();
    println!("(part 1) priority sum of misplaced items: {sum}");

    let sum: u64 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|v| {
            let comp1 = HashSet::<_>::from_iter(v[0].chars());
            let comp2 = HashSet::<_>::from_iter(v[1].chars());
            let comp3 = HashSet::<_>::from_iter(v[2].chars());
            comp1
                .intersection(&comp2)
                .map(|x| *x)
                .collect::<HashSet<_>>()
                .intersection(&comp3)
                .fold(0, |acc, x| acc + (priority_mapping(x) as u64))
        })
        .sum();
    println!("(part 2) priority sum of badges: {sum}");
}
