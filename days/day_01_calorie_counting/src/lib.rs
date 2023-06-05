use std::cmp::Ord;
use std::fmt::Debug;
use std::iter::Sum;
use std::str::FromStr;

/// "one-liner" functional solution
pub fn highest_group_sum<T>(input: &str) -> Result<T, ()>
where
    T: Copy + FromStr + Ord + Sum,
    <T as FromStr>::Err: Debug,
{
    Ok(*input
        .trim()
        .split("\n\n")
        .map(|i| {
            i.lines()
                .map(|n| <T>::from_str(n).expect("failed to parse input to number"))
                .sum()
        })
        .collect::<Vec<T>>()
        .iter()
        .max()
        .unwrap())
}

////////////////////////////////////////////////////////////////////////////////
// *or* do domain modeling type thing
////////////////////////////////////////////////////////////////////////////////
pub fn sort_elfs_by_calories<T: Copy + Sum + Ord>(mut elfs: Vec<Elf<T>>) -> Vec<Elf<T>> {
    elfs.sort_by(|e1, e2| e1.calories().cmp(&e2.calories()));
    elfs
}

#[derive(Clone, Debug)]
pub struct Elf<T> {
    food: Vec<T>,
}

impl<T> Elf<T>
where
    T: Copy + FromStr,
{
    pub fn from_str(input: &str) -> Self
    where
        <T as FromStr>::Err: Debug,
    {
        let food = input
            .trim()
            .lines()
            .map(|l| <T>::from_str(l).expect("failed to parse elf food list"))
            .collect();
        Self { food }
    }
}

impl<T> Elf<T>
where
    T: Copy + Sum,
{
    pub fn calories(&self) -> T {
        self.food.iter().copied().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_from_str() {
        let string = format!("1\n2\n3\n\n");
        let elf = Elf::<u32>::from_str(&string);
        assert_eq!(elf.calories(), (1 + 2 + 3));
    }

    #[test]
    fn elf_comp() {
        let elf_small = Elf {
            food: vec![1000_u32],
        };
        let elf_big = Elf {
            food: vec![10000_u32],
        };

        let mut elfs = vec![elf_small.clone(), elf_big.clone()];
        assert_eq!(
            10000_u32,
            sort_elfs_by_calories(elfs).last().unwrap().calories(),
        );
    }

    #[test]
    fn func_highest_group_sum() {
        let input =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        assert_eq!(Ok(24000), highest_group_sum::<u32>(&input));
    }
}
