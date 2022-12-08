fn part1() -> u32 {
    let datastream = read_input();
    for start in 0..(datastream.len() - 4) {
        let end = start + 4;
        if unique(datastream[start..end].as_bytes()) {
            return end as u32;
        }
    }
    unreachable!()
}

fn part2() -> u32 {
    let datastream = read_input();
    for start in 0..(datastream.len() - 14) {
        let end = start + 14;
        if unique(datastream[start..end].as_bytes()) {
            return end as u32;
        }
    }
    unreachable!()
}

fn unique(chars: &[u8]) -> bool {
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}

fn read_input() -> &'static str {
    include_str!("../../data/day_06.txt")
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1804);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2508);
    }
}
