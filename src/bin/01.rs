advent_of_code::solution!(1);

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
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
