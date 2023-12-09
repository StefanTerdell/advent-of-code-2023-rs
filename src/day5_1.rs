use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, multispace1, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (rest, numbers) = terminated(
        preceded(tag("seeds: "), separated_list1(space1, digit1)),
        multispace1,
    )(input)?;

    let numbers: Vec<u32> = numbers.iter().map(|n| n.parse().unwrap()).collect();

    Ok((rest, numbers))
}

#[allow(unused)]
#[derive(Debug)]
struct Thruple {
    dest_range_start: u32,
    src_range_start: u32,
    range: u32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Block {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Block {
    fn from_str(input: &str) -> Result<Block> {
        match input {
            "seed-to-soil" => Ok(Block::SeedToSoil),
            "soil-to-fertilizer" => Ok(Block::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Block::FertilizerToWater),
            "water-to-light" => Ok(Block::WaterToLight),
            "light-to-temperature" => Ok(Block::LightToTemperature),
            "temperature-to-humidity" => Ok(Block::TemperatureToHumidity),
            "humidity-to-location" => Ok(Block::HumidityToLocation),
            _ => Err(anyhow!("eh")),
        }
    }
}

fn parse_thruple(input: &str) -> IResult<&str, Thruple> {
    // dbg!("thruple", &input);
    let (rest, values) =
        tuple((digit1, preceded(space1, digit1), preceded(space1, digit1)))(input)?;

    Ok((
        &rest,
        Thruple {
            dest_range_start: values.0.parse().unwrap(),
            src_range_start: values.1.parse().unwrap(),
            range: values.2.parse().unwrap(),
        },
    ))
}

fn parse_block(input: &str) -> IResult<&str, (Block, Vec<Thruple>)> {
    // dbg!("block", &input);
    let (rest, header) = terminated(
        many1(alt((tag("-"), alpha1))),
        terminated(tag(" map:"), line_ending),
    )(input)?;

    let block = Block::from_str(&header.join("")).unwrap();

    let (rest, thruples) = separated_list1(line_ending, parse_thruple)(rest)?;

    Ok((rest, (block, thruples)))
}

fn parse_blocks(input: &str) -> IResult<&str, HashMap<Block, Vec<Thruple>>> {
    // dbg!("blocks", &input);
    let (rest, list) = separated_list1(multispace1, parse_block)(input)?;

    Ok((rest, list.into_iter().collect()))
}

fn parse(input: &str) -> Result<(Vec<u32>, HashMap<Block, Vec<Thruple>>)> {
    // dbg!("parse", &input);
    let (rest, seeds) = parse_seeds(input).map_err(|e| e.to_owned())?;
    let (_, blocks) = parse_blocks(rest).map_err(|e| e.to_owned())?;

    Ok((seeds, blocks))
}

fn offset_seed(seed: u32, entry: &Thruple) -> u32 {
    if seed >= entry.dest_range_start && seed < entry.dest_range_start + entry.range {
        return seed + entry.dest_range_start - entry.src_range_start;
    }

    seed
}

fn process(input: &str) -> Result<usize> {
    let (seeds, blocks) = parse(input)?;

    // seeds.iter().map(|seed| {
    //     blocks
    //         .iter()
    //         .fold(seed, |seed, block| offset_seed(seed, block.1))
    // })

    Ok(35)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(process(input)?, 35);

        Ok(())
    }

    #[test]
    fn test_seeds() -> Result<()> {
        let input = "seeds: 79 14 55 13

";

        assert_eq!(parse_seeds(input)?, ("", vec![79, 14, 55, 13]));

        Ok(())
    }
}

pub fn main() -> Result<()> {
    let input = "";

    println!("5.1: {}", process(input)?);

    Ok(())
}
