fn part1() -> u32 {
    let _input = parse_input();
    0
}

fn part2() -> u32 {
    let _input = parse_input();
    0
}

fn parse_input() -> Vec<()> {
    let input = include_str!("../../data/day_01.txt");

    peg::parser! {
        grammar parser() for str {
            pub(crate) rule input() -> Vec<()>
                = "TODO" { todo!() }
        }
    }

    parser::input(input).unwrap()
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
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
