// LINK: https://adventofcode.com/2023/day/1
// DATA: https://adventofcode.com/2023/day/1/input
const DIGITS: &[&str; 18] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

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
    fn get_sum(&self) -> u32 {
        let (first, last) = self.get_pair();
        (first * 10) + last
    }
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
}

struct Part2Parser {
    digits: Vec<(usize, usize)>,
}

impl LineParser for Part2Parser {
    fn parse(line: &str) -> Self {
        let mut digits = vec![];

        for (di, digit) in DIGITS.iter().enumerate() {
            for (i, _) in line.match_indices(digit) {
                let d = if di < 9 {
                    digit
                } else {
                    DIGITS[di.rem_euclid(9)]
                };
                let d = d.parse::<usize>().unwrap();
                digits.push((d, i));
            }
        }

        digits.sort_by_key(|&(_, i)| i);

        Self { digits }
    }

    fn get_pair(&self) -> (u32, u32) {
        let first = match self.digits.first() {
            Some((td, _)) => *td,
            _ => 0,
        };

        let last = match self.digits.last() {
            Some((td, _)) => *td,
            _ => 0,
        };

        (first as u32, last as u32)
    }
}
