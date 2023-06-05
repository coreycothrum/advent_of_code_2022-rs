use day_1_calorie_counting::{sort_elfs_by_calories, Elf};
use std::fs;

fn main() {
    let mut elfs = fs::read_to_string("data/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|f| Elf::<u32>::from_str(f))
        .collect();
    println!(
        "most calories owned by single elf: {}",
        sort_elfs_by_calories(elfs).last().unwrap().calories()
    );
}
