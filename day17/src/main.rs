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

    let mut part_1: Vec<u32> = vec![0b111111111];

    // 2022 == number of rocks to iterate through
    for _ in 0..2022 {
        let shape = s_iter.next().unwrap();
        let mut vec = shape.to_vec();

        let empty = vec![0b100000001; 3 + vec.len()];
        part_1.extend(empty);

        let mut idx = part_1.len() - vec.len();

        loop {
            let dir = d_iter.next().unwrap();
            let slice = Vec::from(&part_1[idx..idx + vec.len()]);
            dir.shift(&mut vec, &slice);

            idx -= 1;

            // If falling further would cause the rock to move
            // through existing rock/ground then break loop.
            if vec
                .iter()
                .zip(part_1[idx..].iter())
                .any(|(a, b)| a & b != 0)
            {
                break;
            }
        }

        // Merge rock with solution
        for i in 0..vec.len() {
            part_1[idx + i + 1] |= vec[i];
        }

        // Remove extra space at "top" of solution;
        while part_1.last() == Some( &0b100000001 ) {
            part_1.pop();
        }

    // And do it all over.
    }

    part_1.iter_mut().for_each(|num| {
        *num ^= 0b100000001;
        *num >>= 1;
    });

    part_1.iter().rev().for_each(|x| {
        let s = format!("{x:07b}");
        for x in s.chars() {
            if x == '1' {
                print!("🟥")
            } else {
                print!("⬛")
            }
        }
        print!("\n");
    });

    // for num in part_1.iter().rev() {
    //     println!("{num:b}");
    // }

    println!("Total lines: {}", part_1.len());

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
            Self::Horizontal => vec![0b000111100],
            Self::Cross => vec![0b000010000, 0b000111000, 0b000010000],
            Self::Corner => vec![0b000111000, 0b000001000, 0b000001000],
            Self::Vertical => vec![0b001000000, 0b001000000, 0b001000000, 0b001000000],
            Self::Square => vec![0b001100000, 0b001100000],
        }
    }
}
