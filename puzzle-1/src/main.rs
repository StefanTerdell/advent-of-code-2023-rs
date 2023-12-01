use anyhow::{anyhow, Result};
use std::{collections::HashMap, fs};

const INPUT_DIRECTORY: &str = "./input";

fn get_digits(line: &str) -> Result<u32> {
    let chars = line.chars();

    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for char in chars {
        if let Some(digit) = char.to_digit(10) {
            if first.is_none() {
                first = Some(digit);
            }

            last = Some(digit);
        }
    }

    let first = first.ok_or(anyhow!("expected digit"))?;
    let last = last.ok_or(anyhow!("expected digit"))?;

    Ok(first * 10 + last)
}

fn sum_lines(lines: &[String]) -> Result<u32> {
    let mut sum = 0;

    for line in lines.iter() {
        sum += get_digits(line)?;
    }

    Ok(sum)
}

fn get_files() -> Result<HashMap<String, Vec<String>>> {
    let entries = fs::read_dir(INPUT_DIRECTORY)?;
    let mut map = HashMap::new();

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            let file = fs::read_to_string(&path)?;
            let lines = file.lines().map(|l| l.to_string()).collect();
            let filename = &path
                .file_name()
                .ok_or(anyhow!("failed extracting filename"))?
                .to_str()
                .ok_or(anyhow!("failed converting to str"))?;

            map.insert(filename.to_string(), lines);
        }
    }

    Ok(map)
}

fn main() -> Result<()> {
    for (file, lines) in get_files()?.iter() {
        println!("{}: {}", file, sum_lines(lines)?);
    }

    Ok(())
}
