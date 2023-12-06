// LINK: https://adventofcode.com/2023/day/1
// DATA: https://adventofcode.com/2023/day/1/input
const SPELLED_DIGITS: &[&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS: &[&str; 9] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub fn run_p1() {
    let document = include_str!("../in-data/d1.txt");
    let mut sum = 0u32;
    for line in document.lines() {
        let parsed_line = Part1Parser::parse(line);
        sum += parsed_line.get_sum();
    }
    println!("sum: {sum}");
}

pub fn run_p2() {
    let document = include_str!("../in-data/d1.txt");
    let mut sum = 0u32;
    for line in document.lines() {
        let parsed_line = Part2Parser::parse(line);
        sum += parsed_line.get_sum();
    }
    println!("sum: {sum}");
}

trait LineParser {
    fn parse(line: &str) -> Self;
    fn get_pair(&self) -> (u32, u32);
    fn get_sum(&self) -> u32;
    fn translate_word_digit(_digit: &str) -> usize {
        0
    }
}

struct Part1Parser {
    digits: Vec<u32>,
}

impl LineParser for Part1Parser {
    fn parse(line: &str) -> Self {
        let digits: Vec<_> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        Self { digits }
    }

    fn get_pair(&self) -> (u32, u32) {
        if self.digits.is_empty() {
            return (0, 0);
        }
        let first = self.digits.first().unwrap();
        let last = self.digits.last().unwrap();
        (*first, *last)
    }

    fn get_sum(&self) -> u32 {
        let (first, last) = self.get_pair();
        (first * 10) + last
    }
}

struct Part2Parser {
    true_digit_indices: Vec<(usize, usize)>,
    word_digit_indices: Vec<(usize, usize)>,
}

impl LineParser for Part2Parser {
    fn parse(line: &str) -> Self {
        let mut true_digit_indices = vec![];
        let mut word_digit_indices = vec![];

        for digit in SPELLED_DIGITS {
            let translated = Self::translate_word_digit(digit);
            for (i, _) in line.match_indices(digit) {
                word_digit_indices.push((translated, i));
            }
        }

        for digit in DIGITS {
            let d = digit.parse::<usize>().unwrap();
            for (i, _) in line.match_indices(digit) {
                true_digit_indices.push((d, i));
            }
        }

        true_digit_indices.sort_by_key(|&(_, i)| i);
        word_digit_indices.sort_by_key(|&(_, i)| i);

        Self {
            true_digit_indices,
            word_digit_indices,
        }
    }

    fn translate_word_digit(digit: &str) -> usize {
        match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        }
    }

    fn get_pair(&self) -> (u32, u32) {
        let true_first = self.true_digit_indices.first();
        let true_last = self.true_digit_indices.last();

        let word_first = self.word_digit_indices.first();
        let word_last = self.word_digit_indices.last();

        let first = match (true_first, word_first) {
            (Some((td, ti)), Some((_, wi))) if ti < wi => *td,
            (_, Some((wd, _))) => *wd,
            (Some((td, _)), _) => *td,
            _ => 0,
        };

        let last = match (true_last, word_last) {
            (Some((td, ti)), Some((_, wi))) if ti > wi => *td,
            (_, Some((wd, _))) => *wd,
            (Some((td, _)), _) => *td,
            _ => 0,
        };

        (first as u32, last as u32)
    }

    fn get_sum(&self) -> u32 {
        let (first, last) = self.get_pair();
        (first * 10) + last
    }
}
