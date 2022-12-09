use std::collections::HashSet;

fn main() {
    let input = parse_input();
    // println!("{:?}", input);
    let mut visited: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(), HashSet::new()];
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited[0].insert(rope[1]);
    visited[1].insert(rope[9]);
    for (dir, tot) in input {
        let dir = match dir {
            'U' => ( 0,  1 ),
            'D' => ( 0, -1 ),
            'L' => (-1,  0 ),
            'R' => ( 1,  0 ),
            _ => unreachable!(),
        };
        for _ in 0..tot {
            rope[0] = (dir.0 + rope[0].0, dir.1 + rope[0].1);
            for i in 1..10 {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                let (x_diff, y_diff) = (head.0 - tail.0, head.1 - tail.1);
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    if tail.0 != head.0 && tail.1 != head.1 {
                        *tail = (
                            tail.0 + x_diff / x_diff.abs(),
                            tail.1 + y_diff / y_diff.abs(),
                        );
                    } else if x_diff.abs() > 1 {
                        *tail = (tail.0 + x_diff / x_diff.abs(), tail.1);
                    } else {
                        *tail = (tail.0, tail.1 + y_diff / y_diff.abs());
                    }
                }
            }
            // println!("head = {:?}, tail = {:?}", head, tail);
            visited[0].insert(rope[1]);
            visited[1].insert(rope[9]);
        }
    }
    println!(
        "Part 1:\n\tTail visited {} locations.\nPart2:\n\tTail visited {} locations.",
        visited[0].len(),
        visited[1].len()
    );
}

fn parse_input() -> Vec<(char, i32)> {
    let input: Vec<_> = include_str!("input.txt").split("\r\n").collect();
    let mut out: Vec<_> = vec![];
    input.iter().for_each(|&x| {
        let mut iter = x.chars();
        let one = iter.next().unwrap();
        iter.next().unwrap();
        let two: String = iter.collect();
        let two: i32 = two.parse().unwrap();
        out.push((one, two.into()));
    });
    out
}
