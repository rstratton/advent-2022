use std::{collections::HashSet, fs};

#[derive(Default, Debug)]
struct Rucksack1 {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl From<&str> for Rucksack1 {
    fn from(s: &str) -> Self {
        let mut rucksack: Rucksack1 = Default::default();
        for (idx, c) in s.chars().enumerate() {
            if idx < s.len() / 2 {
                rucksack.compartment1.insert(c);
            } else {
                rucksack.compartment2.insert(c);
            }
        }
        rucksack
    }
}

fn intersect_all<T>(sets: &[HashSet<T>]) -> HashSet<T>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut sets = sets.iter();
    match sets.next() {
        Some(set) => sets.fold(set.to_owned(), |result, set| {
            result.intersection(set).copied().collect()
        }),
        None => HashSet::new(),
    }
}

fn common_elem<T>(sets: &[HashSet<T>]) -> T
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let intersection = intersect_all(sets);
    intersection.iter().next().unwrap().to_owned()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as u32,
        'A'..='Z' => (c as u8 - b'A' + 27) as u32,
        _ => panic!("Got unexpected char: {}", c),
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<String> {
    fs::read_to_string("data/day_03.txt")
        .expect("File missing")
        .trim()
        .split("\r\n")
        .map(str::to_string)
        .collect()
}

fn part1() -> u32 {
    read_input()
        .iter()
        .map(|items| {
            let rucksack = Rucksack1::from(&items[..]);
            let sets = [rucksack.compartment1, rucksack.compartment2];
            priority(common_elem(&sets))
        })
        .sum()
}

fn part2() -> u32 {
    read_input()
        .chunks(3)
        .map(|group| {
            let item_sets: Vec<HashSet<char>> = group
                .iter()
                .map(|items| HashSet::from_iter(items.chars()))
                .collect();
            priority(common_elem(&item_sets))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8109);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2738);
    }
}
