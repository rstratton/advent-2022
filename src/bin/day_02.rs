use std::fs;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn play(my_action: Action, their_action: Action) -> Outcome {
    if my_action == their_action {
        return Outcome::Draw;
    }

    match (my_action, their_action) {
        (Action::Rock, Action::Scissors) => Outcome::Win,
        (Action::Paper, Action::Rock) => Outcome::Win,
        (Action::Scissors, Action::Paper) => Outcome::Win,
        _ => Outcome::Lose,
    }
}

fn score(my_action: Action, their_action: Action) -> u32 {
    let action_score: u32 = match my_action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    };
    let outcome_score = match play(my_action, their_action) {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    action_score + outcome_score
}

fn read_input() -> Vec<(char, char)> {
    fs::read_to_string("data/day_02.txt")
        .expect("File missing")
        .split("\r\n")
        .map(|line| {
            let mut chars = line.chars().filter(|c| *c != ' ');
            (chars.next().unwrap(), chars.next().unwrap())
        })
        .collect()
}

fn part1() -> u32 {
    read_input()
        .iter()
        .map(|(char1, char2)| {
            let their_action = match char1 {
                'A' => Action::Rock,
                'B' => Action::Paper,
                'C' => Action::Scissors,
                _ => panic!("Unexpected character: {}", char1),
            };
            let my_action = match char2 {
                'X' => Action::Rock,
                'Y' => Action::Paper,
                'Z' => Action::Scissors,
                _ => panic!("Unexpected character: {}", char2),
            };
            score(my_action, their_action)
        })
        .sum()
}

fn necessary_action(their_action: Action, target_outcome: Outcome) -> Action {
    match target_outcome {
        Outcome::Draw => their_action,
        Outcome::Win => match their_action {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissors,
            Action::Scissors => Action::Rock,
        },
        Outcome::Lose => match their_action {
            Action::Rock => Action::Scissors,
            Action::Paper => Action::Rock,
            Action::Scissors => Action::Paper,
        },
    }
}

fn part2() -> u32 {
    read_input()
        .iter()
        .map(|(char1, char2)| {
            let their_action = match char1 {
                'A' => Action::Rock,
                'B' => Action::Paper,
                'C' => Action::Scissors,
                _ => panic!("Unexpected character: {}", char1),
            };
            let target_outcome = match char2 {
                'X' => Outcome::Lose,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => panic!("Unexpected character: {}", char2),
            };
            let my_action = necessary_action(their_action, target_outcome);
            score(my_action, their_action)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 11386);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 13600);
    }
}
