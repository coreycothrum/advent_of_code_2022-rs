use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    let slackers = input.lines().fold(0, |acc, p| {
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

        acc + if set1.is_subset(&set2) || set1.is_superset(&set2) {
            1_u64
        } else {
            0_u64
        }
    });

    println!("elfs w/ nothing to do: {slackers}");
}
