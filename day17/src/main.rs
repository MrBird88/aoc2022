use std::time::Instant;

fn main() {
    let t0 = Instant::now();

    let input = parse_input();
    let s = vec![
        Shape::Horizontal,
        Shape::Cross,
        Shape::Corner,
        Shape::Vertical,
        Shape::Square,
    ];
    let mut s_iter = s.iter().cycle();
    let mut d_iter = input.iter().cycle();

    let mut answer: Vec<u32> = vec![0b111111111];
    // let mut count: u64 = 0;

    // Part 1:
    // 2022 == number of rocks to iterate through

    // Part 2:
    // 1_000_000_000_000 == number of rocks
    for _ in 0..2022 {
        let shape = s_iter.next().unwrap();
        let mut vec = shape.to_vec();

        let empty = vec![0b100000001; 3 + vec.len()];
        answer.extend(empty);

        let mut idx = answer.len() - vec.len();

        loop {
            let dir = d_iter.next().unwrap();
            let slice = Vec::from(&answer[idx..idx + vec.len()]);
            dir.shift(&mut vec, &slice);

            idx -= 1;

            // If falling further would cause the rock to move
            // through existing rock/ground then break loop.
            if vec
                .iter()
                .zip(answer[idx..].iter())
                .any(|(a, b)| a & b != 0)
            {
                break;
            }
        }

        // Merge rock with solution
        for i in 0..vec.len() {
            answer[idx + i + 1] |= vec[i];
        }

        // Remove extra space at "top" of solution;
        while answer.last() == Some(&0b100000001) {
            answer.pop();
        }

        // And do it all over.
    }

    // answer.iter_mut().for_each(|num| {
    //     *num ^= 0b100000001;
    //     *num >>= 1;
    // });

    // answer.iter().rev().for_each(|x| {
    //     let s = format!("{x:07b}");
    //     for x in s.chars() {
    //         if x == '1' {
    //             print!("🟥")
    //         } else {
    //             print!("⬛")
    //         }
    //     }
    //     print!("\n");
    // });

    // for num in answer.iter().rev() {
    //     println!("{num:b}");
    // }

    println!("Total lines: {}", answer.len() - 1);

    println!("Total time: {:?}", Instant::now().duration_since(t0));
}

fn parse_input() -> Vec<Direction> {
    let input = include_str!("input.txt");
    let mut out = vec![];
    for chr in input.chars() {
        let dir = Direction::which(chr);
        out.push(dir);
    }
    out
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn which(arrow: char) -> Direction {
        match arrow {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }

    fn shift(&self, vec: &mut Vec<u32>, slice: &Vec<u32>) {
        match self {
            Self::Left => {
                let mut clone = vec.clone();
                clone.iter_mut().for_each(|num| *num <<= 1);
                if clone.iter().zip(slice).all(|(a, b)| a & b == 0) {
                    vec.iter_mut().for_each(|num| *num <<= 1);
                }
            }
            Self::Right => {
                let mut clone = vec.clone();
                clone.iter_mut().for_each(|num| *num >>= 1);
                if clone.iter().zip(slice).all(|(a, b)| a & b == 0) {
                    vec.iter_mut().for_each(|num| *num >>= 1);
                }
            }
        }
    }
}

// Vestigial enum from a different approach. All it does now is get the vector of the required shape.
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Shape {
    fn to_vec(&self) -> Vec<u32> {
        match self {
            Self::Horizontal => vec![
                0b000111100
                ],

            Self::Cross => vec![
                0b000010000,
                0b000111000,
                0b000010000,
                ],

            Self::Corner => vec![
                0b000111000,
                0b000001000,
                0b000001000,
                ],

            Self::Vertical => vec![
                0b000100000,
                0b000100000,
                0b000100000,
                0b000100000,
                ],

            Self::Square => vec![
                0b000110000,
                0b000110000,
                ],
        }
    }
}
