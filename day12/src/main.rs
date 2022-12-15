use std::{collections::VecDeque, time::Instant};

fn main() {
    let t0 = Instant::now();

    let mut input = parse_input();

    let (start, stop) = parse_start_stop(&input);
    input[start.0][start.1] = b'a';
    input[stop.0][stop.1] = b'z';

    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    visited[start.0][start.1] = true;

    let mut queue: VecDeque<_> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    let mut ans = 0;

    // Part 1
    while let Some(_) = queue.front() {
        let (x1, y1, count) = queue.pop_front().unwrap();

        if (x1, y1) == stop {
            ans = count;
            break;
        }

        let possible_moves = [
            (x1.checked_sub(1), Some(y1)),
            (Some(x1), Some(y1 + 1)),
            (Some(x1 + 1), Some(y1)),
            (Some(x1), y1.checked_sub(1)),
        ];

        let mut valid_moves = vec![];

        for (a, b) in possible_moves {
            match (a, b) {
                (Some(x), Some(y)) => {
                    if x < input.len() && y < input[0].len() {
                        if input[x][y] <= input[x1][y1] + 1 {
                            valid_moves.push((x, y, count + 1));
                        }
                    }
                }

                (_, _) => {
                    continue;
                }
            }
        }

        for (x, y, count) in valid_moves {
            if !visited[x][y] {
                visited[x][y] = true;
                queue.push_back((x, y, count));
            }
        }
    }

    let mut vec_a = vec![];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == b'a' {
                vec_a.push((i, j));
            }
        }
    }

    let mut vec_count = vec![];

    for coord in vec_a {
        let mut visited = vec![vec![false; input[0].len()]; input.len()];
        visited[coord.0][coord.1] = true;

        queue.clear();
        queue.push_back((coord.0, coord.1, 0));

        // Part 2
        while let Some(_) = queue.front() {
            let (x1, y1, count) = queue.pop_front().unwrap();

            if (x1, y1) == (stop.0, stop.1) {
                println!("break");
                vec_count.push(count);
                break;
            }

            let possible_moves = [
                (x1.checked_sub(1), Some(y1)),
                (Some(x1), Some(y1 + 1)),
                (Some(x1 + 1), Some(y1)),
                (Some(x1), y1.checked_sub(1)),
            ];

            let mut valid_moves = vec![];

            for (a, b) in possible_moves {
                match (a, b) {
                    (Some(x), Some(y)) => {
                        if x < input.len() && y < input[0].len() {
                            if input[x][y] <= input[x1][y1] + 1 {
                                valid_moves.push((x, y));
                            }
                        }
                    }

                    (_, _) => {
                        continue;
                    }
                }
            }

            for (x, y) in valid_moves {
                if !visited[x][y] {
                    visited[x][y] = true;
                    queue.push_back((x, y, count + 1));
                }
            }
        }
    }

    let ans2 = vec_count.iter().min().unwrap();
    println!("Part 1:\n\tSteps: {}\n", ans);
    println!("Part 2:\n\tSteps: {}\n", ans2);

    println!("Time taken: {:?}", Instant::now().duration_since(t0));
}

fn parse_input() -> Vec<Vec<u8>> {
    let input = include_str!("input.txt").split("\r\n");
    let mut out = vec![];
    for line in input {
        let temp: Vec<_> = line.chars().collect();
        out.push(temp);
    }
    out.iter()
        .map(|vec| vec.iter().map(|c| *c as u8).collect())
        .collect()
}

fn parse_start_stop(input: &Vec<Vec<u8>>) -> ((usize, usize), (usize, usize)) {
    let (mut start, mut stop) = ((0, 0), (0, 0));
    let (mut f_start, mut f_stop) = (false, false);

    for idx in 0..input.len() {
        let pos_start = input[idx].iter().position(|x| *x == b'S');
        let pos_stop = input[idx].iter().position(|x| *x == b'E');

        if let Some(pos) = pos_start {
            start = (idx, pos);
            f_start = true;
        }
        if let Some(pos) = pos_stop {
            stop = (idx, pos);
            f_stop = true;
        }
        if f_start && f_stop {
            break;
        }
    }
    (start, stop)
}
