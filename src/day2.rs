use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
fn parse_input_part1(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| line.split_whitespace().map(|num| num.parse().unwrap()).collect()).collect()
}

#[aoc_generator(day2, part2)]
fn parse_input_part2(input: &str) -> Vec<Vec<usize>> {
    parse_input_part1(input)
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<usize>]) -> usize {
    let mut safe_levels_count = 0;

    for report in input {
        if is_safe(report) {
            safe_levels_count += 1;
        }
    }

    safe_levels_count
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    let mut safe_levels_count = 0;

    for report in input {
        if is_safe(report) {
            safe_levels_count += 1;
        }
        else {
            // Activate the Problem Dampener. Remove one of the levels from the report, starting
            // with the first one, and check if the report is safe.
            for i in 0..report.len() {
                let mut modified_report = report.to_owned();
                modified_report.remove(i);

                if is_safe(&modified_report) {
                    safe_levels_count += 1;
                    break;
                }
            }
        }
    }

    safe_levels_count
}

fn is_safe(report: &[usize]) -> bool {
    const ALLOWED_LEVEL_DIFFERENCES: [[isize; 3]; 2] = [[1, 2, 3], [-1, -2, -3]];

    // Calculate the difference between each subsequent number in the report.
    let mut differences = Vec::new();
    for i in 0..report.len() - 1 {
        let difference: isize = report[i] as isize - report[i + 1] as isize;
        differences.push(difference);
    }

    for allowed_range in &ALLOWED_LEVEL_DIFFERENCES {
        let mut is_safe = true;
        for difference in &differences {
            if !allowed_range.contains(difference) {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(4, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
