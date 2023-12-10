use std::ops::Range;

const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum SearchType {
    Parts,
    Gears,
}

struct EngineSearcher<'a> {
    lines: Vec<&'a str>,
    ignores: &'static [char],
    sum: u32,
}

impl<'a> EngineSearcher<'a> {
    fn new(lines: Vec<&'a str>, ignores: &'static [char]) -> Self {
        Self {
            lines,
            ignores,
            sum: 0,
        }
    }

    fn search(&mut self, search_type: SearchType) {
        for (line_idx, line) in self.lines.iter().enumerate() {
            let mut adjacent_numbers = vec![];
            let line_indices = self.find_line_indices(line_idx);
            for (char_idx, char) in line.chars().enumerate() {
                // this means we have found a "symbol" character
                if !self.ignores.contains(&char) {
                    let numbers = self.discover_adjacent_numbers(line_idx, &line_indices, char_idx);
                    adjacent_numbers.append(&mut numbers.clone());
                    Self::process_numbers(&mut self.sum, numbers, &search_type);
                }
            }
        }
    }

    fn process_numbers(sum: &mut u32, numbers: Vec<u32>, search_type: &SearchType) {
        match search_type {
            SearchType::Parts => {
                for n in numbers {
                    *sum += n;
                }
            }
            SearchType::Gears if numbers.len() == 2 => {
                *sum += numbers[0] * numbers[1];
            }
            _ => {}
        }
    }

    fn find_line_indices(&self, line_idx: usize) -> Vec<usize> {
        if line_idx == 0 {
            vec![line_idx, line_idx + 1]
        } else if line_idx == self.lines.len() - 1 {
            vec![line_idx - 1, line_idx]
        } else {
            vec![line_idx - 1, line_idx, line_idx + 1]
        }
    }

    fn find_char_indices(
        &self,
        char_idx: usize,
        line_len: usize,
        is_current_line: bool,
    ) -> Vec<usize> {
        let mut indices = vec![];
        if let Some(idx) = char_idx.checked_sub(1) {
            indices.push(idx);
        }
        if !is_current_line {
            indices.push(char_idx);
        }
        if char_idx < line_len - 1 {
            indices.push(char_idx + 1);
        }
        indices
    }

    fn discover_adjacent_numbers(
        &self,
        line_idx: usize,
        line_indices: &Vec<usize>,
        char_idx: usize,
    ) -> Vec<u32> {
        let mut numbers = vec![];
        let mut prev_range = None;
        let mut prev_line = None;
        for idx in line_indices {
            let line = self.lines[*idx];
            let chars: Vec<_> = line.chars().collect();
            let char_indices = self.find_char_indices(char_idx, line.len(), *idx == line_idx);
            for c in char_indices {
                if NUMBERS.contains(&chars[c]) {
                    let num =
                        Self::resolve_adjacent_digits(&chars, c, &mut prev_range, prev_line, *idx);
                    prev_line = Some(*idx);
                    numbers.push(num);
                }
            }
        }
        numbers.into_iter().filter(|n| *n != 0).collect()
    }

    fn resolve_adjacent_digits(
        chars: &Vec<char>,
        char_idx: usize,
        prev_range: &mut Option<Range<usize>>,
        prev_line: Option<usize>,
        line: usize,
    ) -> u32 {
        let mut start = 0;
        let mut end = 0;
        for i in (0..char_idx).rev() {
            if !NUMBERS.contains(&chars[i]) {
                start = i + 1;
                break;
            }
        }
        for i in char_idx..chars.len() {
            if !NUMBERS.contains(&chars[i]) {
                end = i - 1;
                break;
            }
        }

        // handle the case of numbers ending at line end
        if start > 0 && end == 0 {
            end = chars.len() - 1;
        }

        if let Some(r) = prev_range {
            if let Some(i) = prev_line {
                if (r.contains(&start) || r.contains(&end)) && line == i {
                    return 0;
                }
            }
        }
        if let Some(number) = chars.get(start..=end) {
            let number: String = number.iter().collect();
            *prev_range = Some(start..end);
            return number.parse::<u32>().unwrap();
        }

        *prev_range = None;
        0
    }
}

pub mod p1 {
    use crate::d3::{EngineSearcher, SearchType};

    pub fn run() {
        let input_data = include_str!("../in-data/d3.txt");
        let mut searcher = EngineSearcher::new(
            input_data.lines().collect(),
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '\n'],
        );
        searcher.search(SearchType::Parts);
        println!("sum: {}", searcher.sum);
    }
}

pub mod p2 {
    use crate::d3::{EngineSearcher, SearchType};

    pub fn run() {
        let input_data = include_str!("../in-data/d3.txt");
        let mut searcher = EngineSearcher::new(
            input_data.lines().collect(),
            &[
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '\n', '#', '$', '%', '&',
                '+', '-', '/', '=', '@',
            ],
        );
        searcher.search(SearchType::Gears);
        println!("sum: {}", searcher.sum);
    }
}
