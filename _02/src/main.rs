// #[derive(PartialEq)]
// enum RockPaperScissors {
//     Rock,
//     Paper,
//     Scissors,
// }

// impl From<u8> for RockPaperScissors {
//     fn from(b: u8) -> Self {
//         match b {
//             b'A' | b'X' => RockPaperScissors::Rock,
//             b'B' | b'Y' => RockPaperScissors::Paper,
//             b'C' | b'Z' => RockPaperScissors::Scissors,
//             _ => panic!("Failed to parse RockPaperScissors!"),
//         }
//     }
// }

// fn calc_score(me: RockPaperScissors, opponent: RockPaperScissors) -> u32 {
//     use RockPaperScissors::*;

//     let res = match &me {
//         Rock => 1,
//         Paper => 2,
//         Scissors => 3,
//     };

//     res + match (me, opponent) {
//         (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
//         (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => 0,
//         (m, o) if m == o => 3,
//         _ => unreachable!(),
//     }
// }

// fn main() {
//     let input = std::fs::read_to_string("input").expect("Failed to read input!");

//     let score: u32 = input
//         .trim()
//         .split('\n')
//         .map(|game| {
//             calc_score(
//                 game.bytes().nth(2).unwrap().into(),
//                 game.bytes().nth(0).unwrap().into(),
//             )
//         })
//         .sum();

//     println!("{}", score);
// }

#[derive(PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for RockPaperScissors {
    fn from(b: u8) -> Self {
        match b {
            b'A' => RockPaperScissors::Rock,
            b'B' => RockPaperScissors::Paper,
            b'C' => RockPaperScissors::Scissors,
            _ => panic!("Failed to parse RockPaperScissors!"),
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<u8> for Outcome {
    fn from(b: u8) -> Self {
        match b {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("Failed to parse Outcome!"),
        }
    }
}

fn calc_score(opponent: RockPaperScissors, outcome: Outcome) -> u32 {
    use Outcome::*;
    use RockPaperScissors::*;

    let res = match &outcome {
        Lose => 0,
        Draw => 3,
        Win => 6,
    };

    let me = match (opponent, outcome) {
        (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
        (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
        (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors,
    };

    res + match &me {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input!");

    let score: u32 = input
        .trim()
        .split('\n')
        .map(|game| {
            calc_score(
                game.bytes().nth(0).unwrap().into(),
                game.bytes().nth(2).unwrap().into(),
            )
        })
        .sum();

    println!("{}", score);
}
