// LINK: https://adventofcode.com/2023/day/1
// DATA: https://adventofcode.com/2023/day/1/input
pub fn run() {
    let document = include_str!("../in-data/d1.txt");
    let mut sum = 0u32;
    for line in document.lines() {
        let chars: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        let first = match chars.first() {
            Some(c) => c.to_digit(10).unwrap() * 10,
            None => 0u32,
        };
        let last = match chars.last() {
            Some(c) => c.to_digit(10).unwrap(),
            None => 0u32,
        };
        let combined = first + last;
        sum += combined;
    }
    println!("sum: {sum}");
}
