use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
fn parse_input_part1(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    (left_list, right_list)
}

#[aoc_generator(day1, part2)]
fn parse_input_part2(input: &str) -> (Vec<usize>, Vec<usize>) {
    parse_input_part1(input)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (mut left_list, mut right_list) = input.to_owned();

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        sum += left.abs_diff(*right);
    }
    sum
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (left_list, right_list) = input;

    let mut similarity_score = 0;

    for left in left_list {
        let count = right_list.iter().filter(|&right| *right == *left).count();
        if count > 0 {
            similarity_score += left * count;
        }
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = (
            vec![3, 4, 2, 1, 3, 3],
            vec![4, 3, 5, 3, 9, 3],
        );

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(11, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(31, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
