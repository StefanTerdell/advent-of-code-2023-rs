use anyhow::Result;
use std::{collections::HashSet, fs};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    col: usize,
}

fn process(input: &str) -> u32 {
    let mut numbers = vec![];
    let mut symbols = HashSet::new();

    for (row, line) in input.lines().into_iter().enumerate() {
        let mut number: Option<u32> = None;

        for (col, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number = Some(number.unwrap_or_default() * 10 + digit);
            } else {
                if let Some(value) = number {
                    numbers.push(Number { value, row, col });
                    number = None;
                }

                if char != '.' {
                    symbols.insert(Point { row, col });
                }
            }
        }

        if let Some(value) = number {
            numbers.push(Number {
                value,
                row,
                col: line.len(),
            });
        }
    }

    numbers
        .iter()
        .filter_map(|n| {
            let digits = (n.value.checked_ilog10().unwrap_or_default() + 1) as usize;

            for rel_col in 0..digits + 2 {
                for rel_row in 0..3 {
                    let offset_col = rel_col + n.col;
                    let offset_row = rel_row + n.row;

                    if offset_col > digits && offset_row > 0 {
                        if symbols.contains(&Point {
                            row: offset_row - 1,
                            col: offset_col - digits - 1,
                        }) {
                            return Some(n.value);
                        }
                    }
                }
            }

            None
        })
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sum = process(&input);

    println!("Sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(process(input), 4361);
    }

    #[test]
    fn literal_edge_case() {
        assert_eq!(
            process(
                "1......1
.....!10
1......1"
            ),
            10
        );
    }
}
