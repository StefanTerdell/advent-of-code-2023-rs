use anyhow::{bail, Result};
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GameParser;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Game {
    id: u32,
    sets: Vec<Box<Set>>,
}

impl Game {
    fn ok(&self) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(content: String) -> Result<Vec<Game>> {
    let parsed = GameParser::parse(Rule::file, &content)?;
    let mut games = vec![];

    for file in parsed {
        match file.as_rule() {
            Rule::file => {
                for line in file.into_inner() {
                    match line.as_rule() {
                        Rule::line => {
                            let mut game = Game {
                                id: 0,
                                sets: Vec::new(),
                            };

                            for block in line.into_inner() {
                                match block.as_rule() {
                                    Rule::id => {
                                        for part in block.into_inner() {
                                            match part.as_rule() {
                                                Rule::int => {
                                                    game.id = part.as_span().as_str().parse()?
                                                }
                                                x => bail!("expected int, got {:?}", x),
                                            }
                                        }
                                    }
                                    Rule::set => {
                                        let mut set = Set {
                                            red: 0,
                                            green: 0,
                                            blue: 0,
                                        };

                                        for part in block.into_inner() {
                                            match part.as_rule() {
                                                Rule::color_block => {
                                                    let mut seen_value: Option<u32> = None;

                                                    for color_or_int in part.into_inner() {
                                                        match color_or_int.as_rule() {
                                                            Rule::color => {
                                                                let value = seen_value
                                                                    .expect("Missing value");

                                                                match color_or_int
                                                                    .as_span()
                                                                    .as_str()
                                                                {
                                                                    "red" => {
                                                                        set.red += value;
                                                                    }
                                                                    "green" => {
                                                                        set.green += value;
                                                                    }
                                                                    "blue" => {
                                                                        set.blue += value;
                                                                    },
                                                x => bail!("Expected \"red\", \"green\" or \"blue\", got \"{}\"", x),
                                                                }

                                                                seen_value = None;
                                                            }
                                                            Rule::int => {
                                                                seen_value = Some(
                                                                    color_or_int
                                                                        .as_span()
                                                                        .as_str()
                                                                        .parse()?,
                                                                );
                                                            }
                                                            x => bail!(
                                                                "Expected color or int, got {:?}",
                                                                x
                                                            ),
                                                        }
                                                    }
                                                }
                                                x => bail!("Expected color_block, got {:?}", x),
                                            }
                                        }

                                        game.sets.push(Box::new(set));
                                    }
                                    x => bail!("Expected set or ID, got {:?}", x),
                                }
                            }

                            games.push(game);
                        }
                        Rule::EOI => return Ok(games),
                        x => bail!("Expected line, got {:?}", x),
                    }
                }
            }
            x => bail!("Expected file or EOI, got {:?}", x),
        }
    }

    bail!("")
}

fn sum(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            let max = g.sets.iter().fold(
                Set {
                    red: 1,
                    green: 1,
                    blue: 1,
                },
                |mut acc, curr| {
                    if curr.red > acc.red {
                        acc.red = curr.red;
                    }
                    if curr.green > acc.green {
                        acc.green = curr.green;
                    }
                    if curr.blue > acc.blue {
                        acc.blue = curr.blue;
                    }

                    acc
                },
            );

            max.red * max.green * max.blue
        })
        .sum()
}

fn main() -> Result<()> {
    let content = fs::read_to_string("input")?;
    let parsed = parse(content.to_string())?;
    let result = sum(&parsed);

    println!("sum: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        assert_eq!(
            parse(input.to_string())?,
            vec![
                Game {
                    id: 1,
                    sets: vec![
                        Box::new(Set {
                            blue: 3,
                            red: 4,
                            green: 0
                        }),
                        Box::new(Set {
                            red: 1,
                            green: 2,
                            blue: 6,
                        }),
                        Box::new(Set {
                            red: 0,
                            green: 2,
                            blue: 0,
                        })
                    ]
                },
                Game {
                    id: 2,
                    sets: vec![
                        Box::new(Set {
                            blue: 1,
                            green: 2,
                            red: 0,
                        }),
                        Box::new(Set {
                            red: 1,
                            green: 3,
                            blue: 4,
                        }),
                        Box::new(Set {
                            red: 0,
                            green: 1,
                            blue: 1,
                        })
                    ]
                },
                Game {
                    id: 3,
                    sets: vec![
                        Box::new(Set {
                            green: 8,
                            blue: 6,
                            red: 20,
                        }),
                        Box::new(Set {
                            blue: 5,
                            red: 4,
                            green: 13,
                        }),
                        Box::new(Set {
                            red: 1,
                            green: 5,
                            blue: 0,
                        })
                    ]
                },
                Game {
                    id: 4,
                    sets: vec![
                        Box::new(Set {
                            green: 1,
                            red: 3,
                            blue: 6,
                        }),
                        Box::new(Set {
                            green: 3,
                            red: 6,
                            blue: 0,
                        }),
                        Box::new(Set {
                            red: 14,
                            blue: 15,
                            green: 3,
                        })
                    ]
                },
                Game {
                    id: 5,
                    sets: vec![
                        Box::new(Set {
                            red: 6,
                            blue: 1,
                            green: 3,
                        }),
                        Box::new(Set {
                            blue: 2,
                            red: 1,
                            green: 2,
                        }),
                    ]
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_summing() {
        let input = vec![
            Game {
                id: 1,
                sets: vec![
                    Box::new(Set {
                        blue: 3,
                        red: 4,
                        green: 0,
                    }),
                    Box::new(Set {
                        red: 1,
                        green: 2,
                        blue: 6,
                    }),
                    Box::new(Set {
                        red: 0,
                        green: 2,
                        blue: 0,
                    }),
                ],
            },
            Game {
                id: 2,
                sets: vec![
                    Box::new(Set {
                        blue: 1,
                        green: 2,
                        red: 0,
                    }),
                    Box::new(Set {
                        red: 1,
                        green: 3,
                        blue: 4,
                    }),
                    Box::new(Set {
                        red: 0,
                        green: 1,
                        blue: 1,
                    }),
                ],
            },
            Game {
                id: 3,
                sets: vec![
                    Box::new(Set {
                        green: 8,
                        blue: 6,
                        red: 20,
                    }),
                    Box::new(Set {
                        blue: 5,
                        red: 4,
                        green: 13,
                    }),
                    Box::new(Set {
                        red: 1,
                        green: 5,
                        blue: 0,
                    }),
                ],
            },
            Game {
                id: 4,
                sets: vec![
                    Box::new(Set {
                        green: 1,
                        red: 3,
                        blue: 6,
                    }),
                    Box::new(Set {
                        green: 3,
                        red: 6,
                        blue: 0,
                    }),
                    Box::new(Set {
                        red: 14,
                        blue: 15,
                        green: 3,
                    }),
                ],
            },
            Game {
                id: 5,
                sets: vec![
                    Box::new(Set {
                        red: 6,
                        blue: 1,
                        green: 3,
                    }),
                    Box::new(Set {
                        blue: 2,
                        red: 1,
                        green: 2,
                    }),
                ],
            },
        ];

        assert_eq!(sum(&input), 2286);
    }
}
