advent_of_code::solution!(2);

use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, Debug, PartialEq, FromStr, Eq, Hash)]
#[display(style = "snake_case")]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{count} {color}")]
struct CubePull {
    count: u32,
    color: Color,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Game {num}")]
struct Game {
    num: u32,
    #[from_str(default)]
    pulls: Vec<Vec<CubePull>>,
}

fn parse_pull(input: &str) -> Vec<CubePull> {
    input
        .split(',')
        .map(|s| s.trim().parse::<CubePull>().unwrap())
        .collect()
}

fn parse_game(input: &str) -> Game {
    let colon_index = input.find(':').unwrap();
    let mut game: Game = input[..colon_index].parse().unwrap();
    game.pulls = input[colon_index + 1..]
        .split(';')
        .map(|s| parse_pull(s.trim()))
        .collect();
    game
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .trim()
        .split('\n')
        .map(parse_game)
        .collect::<Vec<Game>>();
    let mut valid_games = 0;
    for g in games {
        let mut valid = true;
        for p in g.pulls {
            if valid {
                for cubep in p {
                    if valid {
                        match cubep {
                            CubePull {
                                count,
                                color: Color::Red,
                            } => valid = count <= 12,
                            CubePull {
                                count,
                                color: Color::Green,
                            } => valid = count <= 13,
                            CubePull {
                                count,
                                color: Color::Blue,
                            } => valid = count <= 14,
                        }
                    }
                }
            }
        }
        if valid {
            valid_games += g.num;
        }
    }
    Some(valid_games)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .trim()
        .split('\n')
        .map(parse_game)
        .collect::<Vec<Game>>();
    let mut map = HashMap::new();
    let mut res = 0;
    for g in games {
        map.clear();
        for p in g.pulls {
            for cubep in p {
                map.entry(cubep.color)
                    .and_modify(|v: &mut Vec<u32>| v.push(cubep.count))
                    .or_insert(vec![cubep.count]);
            }
        }

        let mut maxes = vec![];
        for (_k, v) in map.iter() {
            maxes.push(v.iter().max().unwrap());
        }
        res += maxes.into_iter().product::<u32>();
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_str = "3 blue".to_string();
        let cube_pull: CubePull = test_str.parse().unwrap();

        assert_eq!(
            cube_pull,
            CubePull {
                count: 3,
                color: Color::Blue
            }
        );

        let test_pull = "3 blue, 4 red";
        let pull = parse_pull(test_pull);
        assert_eq!(
            pull,
            vec![
                CubePull {
                    count: 3,
                    color: Color::Blue
                },
                CubePull {
                    count: 4,
                    color: Color::Red
                }
            ]
        );

        let test_game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(test_game);
        assert_eq!(
            game,
            Game {
                num: 1,
                pulls: vec![
                    vec![
                        CubePull {
                            count: 3,
                            color: Color::Blue
                        },
                        CubePull {
                            count: 4,
                            color: Color::Red
                        }
                    ],
                    vec![
                        CubePull {
                            count: 1,
                            color: Color::Red
                        },
                        CubePull {
                            count: 2,
                            color: Color::Green
                        },
                        CubePull {
                            count: 6,
                            color: Color::Blue
                        },
                    ],
                    vec![CubePull {
                        count: 2,
                        color: Color::Green
                    }],
                ]
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
