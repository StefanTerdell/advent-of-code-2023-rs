use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Race {
    time_ms: u64,
    distance_mm: u64,
}

fn parse_time(input: &str) -> IResult<&str, u64> {
    let (rest, times) = preceded(
        terminated(tag("Time:"), space1),
        separated_list1(space1, digit1),
    )(input)?;

    let time = times.join("").parse().unwrap();

    Ok((rest, time))
}

fn parse_distance(input: &str) -> IResult<&str, u64> {
    let (rest, distances) = preceded(
        terminated(tag("Distance:"), space1),
        separated_list1(space1, digit1),
    )(input)?;

    let distance = distances.join("").parse().unwrap();

    Ok((rest, distance))
}

fn parse_race(input: &str) -> Result<Race> {
    let (_, (time_ms, distance_mm)) =
        separated_pair(parse_time, line_ending, parse_distance)(input).map_err(|e| e.to_owned())?;

    Ok(Race {
        time_ms,
        distance_mm,
    })
}

fn process_race(race: Race) -> u64 {
    let mut ways_to_win = 0;

    for button_hold_time in 0..race.time_ms {
        let remaining_time = race.time_ms - button_hold_time;
        let distance = button_hold_time * remaining_time;

        if distance > race.distance_mm {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

fn process(input: &str) -> Result<u64> {
    let race = parse_race(input)?;
    let total_ways = process_race(race);

    Ok(total_ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(
            parse_race(input)?,
            Race {
                time_ms: 71530,
                distance_mm: 940200
            },
        );

        Ok(())
    }

    #[test]
    fn test_race_processing() {
        let input = Race {
            time_ms: 71530,
            distance_mm: 940200,
        };

        assert_eq!(process_race(input), 71503);
    }
}

pub fn main() -> Result<()> {
    let input = "Time:        51     92     68     90
Distance:   222   2031   1126   1225";

    let result = process(input)?;

    println!("6.2: {}", result);

    Ok(())
}
