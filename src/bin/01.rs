advent_of_code::solution!(1);
use regex::Regex;
use std::collections::HashMap;
fn fix_word_digits(input: &str) -> String {
    let digit_regex = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine|zero)"#).unwrap();
    let replacements: HashMap<&str, &str> = vec![
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "t3hree"),
        ("four", "f4our"),
        ("five", "f5ve"),
        ("six", "s6ix"),
        ("seven", "s7even"),
        ("eight", "e8ight"),
        ("nine", "n9ine"),
        ("zero", "z0ero"),
    ]
    .into_iter()
    .collect();
    let mut lines = vec![];
    for ln in input.split('\n') {
        let mut line = ln.to_string();
        while let Some(m) = digit_regex.find(&line) {
            line = line
                .replace(m.as_str(), replacements.get(m.as_str()).unwrap())
                .to_string();
        }
        lines.push(line);
    }
    lines.join("\n")
}

fn line_digits(input: &str) -> Vec<u32> {
    let mut res = vec![];
    let mut res_vec = vec![];
    for ln in input.split('\n') {
        if ln.is_empty() {
            continue;
        }

        res_vec.clear();
        for ch in ln.chars() {
            if ch.is_numeric() {
                res_vec.push(ch.to_string());
            }
        }

        let first = res_vec.first().unwrap();
        let last = res_vec.last().unwrap();
        res.push(format!("{}{}", first, last).parse::<u32>().unwrap());
    }

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let digits = line_digits(input);
    Some(digits.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let fixed = fix_word_digits(input);
    let digits = line_digits(&fixed);
    Some(digits.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two_fix() {
        let example_two = r#"
two1nine
eighthree
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .trim()
            .to_string();
        let fix_result = fix_word_digits(&example_two);
        let splits = fix_result.split('\n').collect::<Vec<&str>>();
        assert_eq!(
            splits,
            vec![
                "t2wo1n9ine",
                "e8ight3hree",
                "e8ight2wot3hree",
                "abco1ne2t3hreexyz",
                "xt2wo1ne3f4our",
                "4n9inee8ights7even2",
                "zo1ne8ight234",
                "7pqrsts6ixteen"
            ]
        );
    }

    #[test]
    fn test_part_two() {
        let example_two = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .trim()
            .to_string();
        let fix_result = fix_word_digits(&example_two);
        let nums = line_digits(&fix_result);
        assert_eq!(nums, vec![29, 83, 13, 24, 42, 14, 76]);
        let result = part_two(&example_two);
        assert_eq!(result, Some(281));
    }
}
