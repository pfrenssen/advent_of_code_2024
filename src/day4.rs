use aoc_runner_derive::{aoc, aoc_generator};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[aoc_generator(day4, part1)]
fn parse_input_part1(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc_generator(day4, part2)]
fn parse_input_part2(input: &str) -> Vec<Vec<char>> {
    parse_input_part1(input)
}

#[derive(Display, EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn to_vector(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }

    // Check if the direction is valid for the current position.
    fn is_valid(&self, x: isize, y: isize, max_x: isize, max_y: isize) -> bool {
        let (dx, dy) = self.to_vector();
        x + dx >= 0 && x + dx < max_x && y + dy >= 0 && y + dy < max_y
    }
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    const XMAS: &str = "XMAS";
    let mut xmas_count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            // Skip if the current character is not an X.
            if input[y][x] != 'X' {
                continue;
            }
            for direction in Direction::iter() {
                let found = find_word(input, x as isize, y as isize, &direction, XMAS);
                if found {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}

fn find_word(grid: &[Vec<char>], x: isize, y: isize, direction: &Direction, search: &str) -> bool {
    let last_char = search.chars().last().unwrap();
    let (dx, dy) = direction.to_vector();
    let mut curx = x;
    let mut cury = y;

    let mut found = true;
    for c in search.chars() {
        // Check if the current character matches the expected character.
        if grid[cury as usize][curx as usize] != c {
            found = false;
            break;
        }

        // If we reach the end of the string, we found a match.
        if c == last_char {
            break;
        }

        // Check if we can proceed in the current direction.
        if !direction.is_valid(curx, cury, grid.len() as isize, grid[0].len() as isize) {
            found = false;
            break;
        }

        // Move to the next position.
        curx += dx;
        cury += dy;
    }

    found
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    const MAS: &str = "MAS";
    let max_x = input[0].len() as isize;
    let max_y = input.len() as isize;

    let search_directions: Vec<((isize, isize), Direction)> = vec![
        ((-1, -1), Direction::SE),
        ((1, 1), Direction::NW),
        ((-1, 1), Direction::NE),
        ((1, -1), Direction::SW),
    ];
    let mut xmas_count = 0;

    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() -1 {
            // Skip if the current character is not an A.
            if input[y][x] != 'A' {
                continue;
            }
            let mut matches = 0;
            for ((dx, dy), direction) in &search_directions {
                let sx = x as isize + dx;
                let sy = y as isize + dy;

                if sx < 0 || sx >= max_x || sy < 0 || sy >= max_y {
                    continue;
                }

                let found = find_word(input, sx, sy, direction, MAS);
                if found {
                    matches += 1;
                }
            }
            if matches == 2 {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
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
        assert_eq!(18, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(9, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
