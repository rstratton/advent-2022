struct Trees {
    width: usize,
    height: usize,
    heights: Vec<Vec<u8>>,
}

impl Trees {
    fn new(heights: Vec<Vec<u8>>) -> Self {
        let height = heights.len();
        let width = heights[0].len();
        Self {
            width,
            height,
            heights,
        }
    }

    fn is_visible(&self, i: usize, j: usize) -> bool {
        if i == 0 || j == 0 || i == (self.height - 1) || j == (self.width - 1) {
            return true;
        }
        let tree_height = self.heights[i][j];
        (0..i)
            .map(|k| self.heights[k][j])
            .all(|other_height| tree_height > other_height)
            || ((i + 1)..self.height)
                .map(|k| self.heights[k][j])
                .all(|other_height| tree_height > other_height)
            || (0..j)
                .map(|k| self.heights[i][k])
                .all(|other_height| tree_height > other_height)
            || ((j + 1)..self.width)
                .map(|k| self.heights[i][k])
                .all(|other_height| tree_height > other_height)
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        if i == 0 || j == 0 || i == (self.height - 1) || j == (self.width - 1) {
            return 0;
        }
        let tree_height = self.heights[i][j];
        let mut result = {
            let mut result: usize = 0;
            for k in (0..i).rev() {
                result += 1;
                if tree_height <= self.heights[k][j] {
                    break;
                }
            }
            result
        };
        result *= {
            let mut result: usize = 0;
            for k in (i + 1)..self.height {
                result += 1;
                if tree_height <= self.heights[k][j] {
                    break;
                }
            }
            result
        };
        result *= {
            let mut result: usize = 0;
            for k in (0..j).rev() {
                result += 1;
                if tree_height <= self.heights[i][k] {
                    break;
                }
            }
            result
        };
        result *= {
            let mut result: usize = 0;
            for k in (j + 1)..self.width {
                result += 1;
                if tree_height <= self.heights[i][k] {
                    break;
                }
            }
            result
        };
        result
    }
}

fn parse_input() -> Trees {
    Trees::new(
        include_str!("../../data/day_08.txt")
            .split("\r\n")
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    )
}

fn part1() -> usize {
    let trees = parse_input();

    // All trees on the perimeter are visible
    let mut visible_count: usize = (2 * trees.height) + (2 * trees.width) - 4;

    for i in 1..(trees.height - 1) {
        for j in 1..(trees.width - 1) {
            if trees.is_visible(i, j) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn part2() -> usize {
    let trees = parse_input();
    let mut max_scenic_score: usize = 0;

    for i in 0..trees.height {
        for j in 0..trees.width {
            max_scenic_score = max_scenic_score.max(trees.scenic_score(i, j));
        }
    }

    max_scenic_score
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
        assert_eq!(part1(), 1782);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 474606);
    }
}
