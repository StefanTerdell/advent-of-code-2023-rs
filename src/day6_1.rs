use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Race {
    time_ms: u32,
    distance_mm: u32,
}

fn parse_times(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        terminated(tag("Time:"), space1),
        separated_list1(space1, u32),
    )(input)
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        terminated(tag("Distance:"), space1),
        separated_list1(space1, u32),
    )(input)
}

fn parse_races(input: &str) -> Result<Vec<Race>> {
    let (_, (times_ms, distances_ms)) =
        separated_pair(parse_times, line_ending, parse_distance)(input)
            .map_err(|e| e.to_owned())?;

    Ok(times_ms
        .iter()
        .enumerate()
        .map(|(index, time_ms)| Race {
            time_ms: *time_ms,
            distance_mm: distances_ms[index],
        })
        .collect())
}

fn process_races(races: Vec<Race>) -> u32 {
    let mut total_ways = 1;

    for race in races {
        let mut ways_to_win: Option<u32> = None;

        for button_hold_time in 0..race.time_ms {
            let remaining_time = race.time_ms - button_hold_time;
            let distance = button_hold_time * remaining_time;

            if distance > race.distance_mm {
                if let Some(count) = ways_to_win {
                    ways_to_win = Some(count + 1);
                } else {
                    ways_to_win = Some(1);
                }
            }
        }

        if let Some(count) = ways_to_win {
            total_ways *= count;
        }
    }

    total_ways
}

fn process(input: &str) -> Result<u32> {
    let races = parse_races(input)?;
    let total_ways = process_races(races);

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
            parse_races(input)?,
            vec![
                Race {
                    time_ms: 7,
                    distance_mm: 9
                },
                Race {
                    time_ms: 15,
                    distance_mm: 40
                },
                Race {
                    time_ms: 30,
                    distance_mm: 200
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_race_processing() {
        let input = vec![
            Race {
                time_ms: 7,
                distance_mm: 9,
            },
            Race {
                time_ms: 15,
                distance_mm: 40,
            },
            Race {
                time_ms: 30,
                distance_mm: 200,
            },
        ];

        assert_eq!(process_races(input), 288);
    }
}

pub fn main() -> Result<()> {
    let input = "Time:        51     92     68     90
Distance:   222   2031   1126   1225";

    let result = process(input)?;

    println!("6.1: {}", result);

    Ok(())
}
