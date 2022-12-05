#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || self.contains(other)
            || other.contains(self)
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn parse_input() -> Vec<(Range, Range)> {
    let input = include_str!("../../data/day_04.txt");

    peg::parser! {
        grammar ranges_parser() for str {
            pub(crate) rule list() -> Vec<(Range, Range)>
                = p:pair()+ { p }

            rule pair() -> (Range, Range)
                = r1:range() "," r2:range() "\r\n"? { (r1, r2) }

            rule range() -> Range
                = start:number() "-" end:number() { Range::new(start, end) }

            rule number() -> u32
                = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }
        }
    }

    ranges_parser::list(input).unwrap()
}

fn part1() -> u32 {
    let ranges = parse_input();
    ranges
        .iter()
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count() as u32
}

fn part2() -> u32 {
    let ranges = parse_input();
    ranges.iter().filter(|(r1, r2)| r1.overlaps(r2)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 657);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
