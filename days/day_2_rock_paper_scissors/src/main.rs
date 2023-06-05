use day_2_rock_paper_scissors::{Hand, Round};
use std::fs;

fn main() {
    let points = fs::read_to_string("data/input.txt")
        .unwrap()
        .lines()
        .map(|r| {
            let s: Vec<&str> = r.split(' ').collect();
            let player_code = s.last().unwrap().chars().next().unwrap();
            let opponent_code = s.first().unwrap().chars().next().unwrap();
            Round::from_fight(
                Hand::from_code(&player_code),
                Hand::from_code(&opponent_code),
            )
        })
        .fold(0, |acc: u64, x| acc + (x.points_from_round() as u64));

    println!("total points: {points}");
}
