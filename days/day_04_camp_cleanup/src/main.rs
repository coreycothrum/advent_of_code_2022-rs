use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    let (slackers, duplicates) = input.lines().fold((0, 0), |acc, p| {
        let mut p = p.split(',');

        let mut range1 = p.next().unwrap().split("-");
        let mut range2 = p.next().unwrap().split("-");

        let set1 = HashSet::<_>::from_iter(
            <u64>::from_str(range1.next().unwrap()).unwrap()
                ..=<u64>::from_str(range1.next().unwrap()).unwrap(),
        );

        let set2 = HashSet::<_>::from_iter(
            <u64>::from_str(range2.next().unwrap()).unwrap()
                ..=<u64>::from_str(range2.next().unwrap()).unwrap(),
        );

        let slacker = acc.0
            + if set1.is_subset(&set2) || set1.is_superset(&set2) {
                1_u64
            } else {
                0_u64
            };

        let duplicate = acc.1
            + if !set1.is_disjoint(&set2) {
                1_u64
            } else {
                0_u64
            };

        (slacker, duplicate)
    });

    println!("elfs w/ nothing to do: {slackers}");
    println!("pairs w/ that overlap: {duplicates}");
}
