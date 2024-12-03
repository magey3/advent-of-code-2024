advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut n = 0;
    for line in input.lines() {
        let s = line
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_safe(&s) {
            n += 1;
        }
    }
    Some(n)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n = 0;
    for line in input.lines() {
        let s = line
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_safe(&s) {
            n += 1;
        } else {
            for i in 0..s.len() {
                let mut v = Vec::from(&s[0..i]);
                v.extend_from_slice(&s[i + 1..]);
                if is_safe(&v) {
                    n += 1;
                    break;
                }
            }
        }
    }
    Some(n)
}

fn is_safe(s: &[i32]) -> bool {
    let sign = (s[0] - s[1]).signum();
    for i in 1..s.len() {
        let d = s[i - 1] - s[i];
        if !(d.signum() == sign && d.abs() >= 1 && d.abs() <= 3) {
            return false;
        }
    }
    true
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
