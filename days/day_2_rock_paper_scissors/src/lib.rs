pub struct Round {
    player: Hand,
    opponent: Hand,
}

impl Round {
    pub fn from_fight(player: Hand, opponent: Hand) -> Self {
        Self { player, opponent }
    }

    pub fn points_from_round(&self) -> u8 {
        self.player.point_value() + Outcome::from_hands(&self.player, &self.opponent).point_value()
    }

    pub fn opponents_points_from_round(&self) -> u8 {
        self.opponent.point_value()
            + Outcome::from_hands(&self.opponent, &self.player).point_value()
    }
}

pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn point_value(&self) -> u8 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn from_code(code: &char) -> Self {
        match *code {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("invalid code"),
        }
    }
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from_hands(player: &Hand, opponent: &Hand) -> Self {
        match (player, opponent) {
            (&Hand::Rock, &Hand::Rock) => Outcome::Draw,
            (&Hand::Rock, &Hand::Paper) => Outcome::Lose,
            (&Hand::Rock, &Hand::Scissors) => Outcome::Win,
            (&Hand::Paper, &Hand::Rock) => Outcome::Win,
            (&Hand::Paper, &Hand::Paper) => Outcome::Draw,
            (&Hand::Paper, &Hand::Scissors) => Outcome::Lose,
            (&Hand::Scissors, &Hand::Rock) => Outcome::Lose,
            (&Hand::Scissors, &Hand::Paper) => Outcome::Win,
            (&Hand::Scissors, &Hand::Scissors) => Outcome::Draw,
        }
    }

    pub fn point_value(&self) -> u8 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let round1 = Round::from_fight(Hand::from_code(&'Y'), Hand::from_code(&'A'));
        let round2 = Round::from_fight(Hand::from_code(&'X'), Hand::from_code(&'B'));
        let round3 = Round::from_fight(Hand::from_code(&'Z'), Hand::from_code(&'C'));

        assert_eq!(round1.points_from_round(), 8);
        assert_eq!(round2.points_from_round(), 1);
        assert_eq!(round3.points_from_round(), 6);
    }
}
