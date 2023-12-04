use anyhow::Result;
use std::{collections::HashMap, fs};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

fn process(input: &str) -> u32 {
    let mut numbers = HashMap::new();
    let mut gears = vec![];

    for (row, line) in input.lines().into_iter().enumerate() {
        let mut number: Option<u32> = None;

        for (col, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number = Some(number.unwrap_or_default() * 10 + digit);
            } else {
                if let Some(value) = number {
                    let digits = (value.checked_ilog10().unwrap_or_default() + 1) as usize;

                    for offset in 0..digits {
                        numbers.insert(
                            Point {
                                row,
                                col: col - offset - 1,
                            },
                            value,
                        );
                    }

                    number = None;
                }

                if char == '*' {
                    gears.push(Point { row, col });
                }
            }
        }

        if let Some(value) = number {
            let digits = (value.checked_ilog10().unwrap_or_default() + 1) as usize;

            for offset in 0..digits {
                numbers.insert(
                    Point {
                        row,
                        col: line.len() - offset - 1,
                    },
                    value,
                );
            }
        }
    }

    gears
        .iter()
        .filter_map(|n| {
            let mut a = None;
            let mut b = None;

            for rel_col in 0..3 {
                for rel_row in 0..3 {
                    let offset_col = rel_col + n.col;
                    let offset_row = rel_row + n.row;

                    if offset_col > 0 && offset_row > 0 {
                        if let Some(value) = numbers.get(&Point {
                            row: offset_row - 1,
                            col: offset_col - 1,
                        }) {
                            match a {
                                Some(a) => {
                                    if a == *value {
                                        continue;
                                    } else {
                                        match b {
                                            Some(b) => {
                                                if b == *value {
                                                    continue;
                                                } else {
                                                    return None;
                                                }
                                            }
                                            None => b = Some(*value),
                                        }
                                    }
                                }
                                None => a = Some(*value),
                            }
                        }
                    }
                }
            }

            if let Some(a) = a {
                if let Some(b) = b {
                    return Some(a * b);
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

        assert_eq!(process(input), 467835);
    }
}
