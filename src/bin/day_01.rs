use std::fs;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Vec<u32>> {
    fs::read_to_string("data/day_01.txt")
        .expect("File missing")
        .trim()
        .split("\r\n\r\n")
        .map(|inventory_str| {
            inventory_str
                .trim()
                .split("\r\n")
                .map(|calories_str| calories_str.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1() -> u32 {
    let inventories = read_input();
    inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

fn part2() -> u32 {
    let inventories = read_input();
    let mut calorie_sums: Vec<u32> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    calorie_sums.sort();
    calorie_sums[(calorie_sums.len() - 3)..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 74394);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 212836);
    }
}
