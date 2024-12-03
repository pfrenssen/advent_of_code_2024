use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
fn parse_input_part1(input: &str) -> Vec<(usize, usize)> {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect()
}

#[aoc_generator(day3, part2)]
fn parse_input_part2(input: &str) -> Vec<(usize, usize)> {
    // Append a `do()` to the end to catch any unpaired `don't()`s
    let input = format!("{}do()", input);
    let re = regex::Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let input = re.replace_all(&input, "");

    parse_input_part1(&input)
}

#[aoc(day3, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        let expected = vec![(2, 4), (8, 5)];

        assert_eq!(expected, parse_input_part2(get_test_input_part2()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(161, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(48, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn get_test_input_part2<'a>() -> &'a str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }
}
