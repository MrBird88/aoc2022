use std::{collections::VecDeque, fs::File, io::Write, time::Instant};

fn main() {
    let t0 = Instant::now();

    let mut input = parse_input();

    let (start, stop) = parse_start_stop(&input);
    input[start.0][start.1] = b'a';
    input[stop.0][stop.1] = b'z';

    let mut v_input = input.clone();
    v_input[start.0][start.1] = b'S';
    v_input[stop.0][stop.1] = b'E';

    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    visited[start.0][start.1] = true;

    let mut queue: VecDeque<_> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    let mut ans = 0;

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
                        if input[x][y] == input[x1][y1] + 1 ||
                           input[x][y] <= input[x1][y1]
                        {
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
                v_input[x][y] = b'.';
                queue.push_back((x, y, count));
            }
        }
    }

    let mut v_file = File::create("v_input.txt").unwrap();
    for vec in v_input {
        for num in vec {
            write!(v_file, "{}", num as char).unwrap();
        }
        write!(v_file, "\n").unwrap();
    }
    println!("Part 1:\n\tSteps: {}\n", ans);

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
