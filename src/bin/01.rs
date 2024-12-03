use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut as_ = Vec::new();
    let mut bs = Vec::new();

    for line in input.lines() {
        let mut s = line.split("   ");
        let a = s.next().unwrap().parse::<u32>().unwrap();
        let b = s.next().unwrap().parse::<u32>().unwrap();

        as_.push(a);
        bs.push(b);
    }

    as_.sort();
    bs.sort();

    Some(as_.into_iter().zip(bs).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut as_ = Vec::new();

    let mut occur = HashMap::<u32, u32>::new();

    for line in input.lines() {
        let mut s = line.split("   ");
        let a = s.next().unwrap().parse::<u32>().unwrap();
        let b = s.next().unwrap().parse::<u32>().unwrap();

        if let Some(x) = occur.get_mut(&b) {
            *x += 1;
        } else {
            occur.insert(b, 1);
        }

        as_.push(a);
    }

    Some(
        as_.into_iter()
            .map(|a| a * occur.get(&a).cloned().unwrap_or_default())
            .sum(),
    )
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
