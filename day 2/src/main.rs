#![allow(unused_variables, dead_code)]
fn main() {
    let input = include_str!("input.txt").split("\r\n");
    // println!("{:?}", input.collect::<Vec<_>>())
    let mut matches = RPS::new();
    for line in input {
        matches.push(line);
    }
    let answer = matches.score();
    println!("{answer}");
}

struct RPS {
    game: Vec<(String, String)>,
}

impl RPS {
    fn new() -> Self {
        RPS { game: Vec::new() }
    }

    fn push(&mut self, value: &str) {
        if value == "" {
            return;
        }
        let value = Self::parse(value);
        self.game.push(value);
    }

    fn parse(input: &str) -> (String, String) {
        let input: Vec<_> = input.split(" ").collect();
        let (a, b) = (input[0], input[1]);
        (a.to_string(), b.to_string())
    }

    fn value(round: (&str, &str)) -> u64 {
        match round {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u64 {
        let mut score = 0;
        for (a, b) in &self.game {
            let points = Self::value((&a, &b));
            score += points;
        }
        score
    }
}
