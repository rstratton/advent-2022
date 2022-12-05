struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn new(num_stacks: usize) -> Self {
        Crates {
            stacks: vec![Default::default(); num_stacks],
        }
    }

    fn from_layers(layers: Vec<Vec<Option<char>>>) -> Self {
        let num_stacks = layers.first().unwrap().len();
        let mut crates = Crates::new(num_stacks);
        for layer in layers.iter().rev() {
            for (idx, crate_slot) in layer.iter().enumerate() {
                if let Some(c) = crate_slot {
                    crates.stacks[idx].push(*c);
                }
            }
        }
        crates
    }

    fn execute(&mut self, i: &[Instruction], crane: Crane) {
        for instruction in i.iter() {
            match crane {
                Crane::CrateMover9000 => self.execute_instruction_9000(instruction),
                Crane::CrateMover9001 => self.execute_instruction_9001(instruction),
            }
        }
    }

    fn execute_instruction_9000(&mut self, i: &Instruction) {
        for _ in 0..i.quantity {
            let c = self.stacks[(i.src - 1) as usize].pop().unwrap();
            self.stacks[(i.dst - 1) as usize].push(c);
        }
    }

    fn execute_instruction_9001(&mut self, i: &Instruction) {
        let mut crates_to_move: Vec<char> = Vec::new();
        for _ in 0..i.quantity {
            crates_to_move.push(self.stacks[(i.src - 1) as usize].pop().unwrap());
        }
        while let Some(c) = crates_to_move.pop() {
            self.stacks[(i.dst - 1) as usize].push(c);
        }
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

struct Instruction {
    quantity: u32,
    src: u32,
    dst: u32,
}

impl Instruction {
    fn new(quantity: u32, src: u32, dst: u32) -> Self {
        Instruction { quantity, src, dst }
    }
}

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

fn part1() -> String {
    let (mut crates, instructions) = parse_input();
    crates.execute(&instructions, Crane::CrateMover9000);
    crates.top_crates()
}

fn part2() -> String {
    let (mut crates, instructions) = parse_input();
    crates.execute(&instructions, Crane::CrateMover9001);
    crates.top_crates()
}

fn parse_input() -> (Crates, Vec<Instruction>) {
    let input = include_str!("../../data/day_05.txt");

    peg::parser! {
        grammar parser() for str {
            pub(crate) rule parse() -> (Crates, Vec<Instruction>)
                = c:crates() "\r\n\r\n" i:instructions() ![_] { (c, i) }

            rule crates() -> Crates
                = l:crates_layer() ++ "\r\n" "\r\n" crates_index() { Crates::from_layers(l) }

            rule crates_layer() -> Vec<Option<char>>
                = c:crate_slot() ++ " " { c }

            rule crate_slot() -> Option<char>
                = "[" c:$(['A'..='Z']) "]" { Some(c.chars().next().unwrap()) }
                / "   " { None }

            rule crates_index()
                = (" " ['0'..='9'] " ") ++ " "

            rule instructions() -> Vec<Instruction>
                = i:instruction() ++ "\r\n" { i }

            rule instruction() -> Instruction
                = "move " n1:number() " from " n2:number() " to " n3:number() { Instruction::new(n1, n2, n3) }

            rule number() -> u32
                = n:$(['0'..='9']+) { n.parse().unwrap() }
        }
    }

    parser::parse(input).unwrap()
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
        assert_eq!(part1(), "HBTMTBSDC");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "PQTJRSHWS");
    }
}
