use regex::Regex;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();

    let (crates, instructions) = {
        let mut split = input.split("\n\n");
        (split.next().unwrap(), split.next().unwrap())
    };

    let mut stacks = Vec::<Vec<char>>::new();

    let re = Regex::new(r"\[(\p{Alphabetic})\]").unwrap();
    for row in crates.lines() {
        for (idx, item) in row.as_bytes().chunks(4).enumerate() {
            let stack = if let Some(stack) = stacks.get_mut(idx) {
                stack
            } else {
                stacks.push(Vec::<char>::new());
                stacks.last_mut().unwrap()
            };

            for letter in re.captures_iter(std::str::from_utf8(&item).unwrap()) {
                stack.push(<char>::from_str(&letter[1]).unwrap());
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for instruction in re.captures_iter(instructions) {
        let (count, from, to) = (
            <usize>::from_str(&instruction[1]).unwrap(),
            <usize>::from_str(&instruction[2]).unwrap() - 1,
            <usize>::from_str(&instruction[3]).unwrap() - 1,
        );

        let idx = stacks[from].len() - count;
        let mut items: Vec<char> = stacks[from].drain(idx..).collect();
        items.reverse();
        stacks[to].append(&mut items);
    }

    for stack in &mut stacks {
        print!("{}", stack.last().unwrap());
    }
}
