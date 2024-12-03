use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut s = 0;

    for caps in r.captures_iter(input) {
        if let (Some(first), Some(second)) = (caps.get(1), caps.get(2)) {
            s += first.as_str().parse::<u32>().unwrap() * second.as_str().parse::<u32>().unwrap();
        }
    }

    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut pos = 0;
    let mut d = true;
    let mut s = 0;
    while pos < input.len() {
        let new = if !d {
            input[pos..].find("do()").unwrap_or(input.len() - pos)
        } else {
            input[pos..].find("don't()").unwrap_or(input.len() - pos)
        };

        if d {
            for caps in r.captures_iter(&input[pos..pos + new]) {
                if let (Some(first), Some(second)) = (caps.get(1), caps.get(2)) {
                    s += first.as_str().parse::<u32>().unwrap()
                        * second.as_str().parse::<u32>().unwrap();
                }
            }
        }

        pos += new;
        d = !d;
    }

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
