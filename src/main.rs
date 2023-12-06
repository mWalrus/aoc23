mod d1;
mod d2;

use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, ValueEnum)]
enum Day {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
}
#[derive(Clone, Copy, ValueEnum, Default)]
enum Part {
    #[default]
    P1,
    P2,
    P3,
    P4,
    P5,
}

#[derive(Parser)]
struct Args {
    #[clap(required = true, about)]
    day: Day,
    #[clap(required = false, about)]
    part: Option<Part>,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part.unwrap_or_default()) {
        (Day::D1, Part::P1) => d1::p1::run(),
        (Day::D1, Part::P2) => d1::p2::run(),
        (Day::D2, Part::P1) => d2::p1::run(),
        (Day::D2, Part::P2) => d2::p2::run(),
        _ => {}
    }
}
