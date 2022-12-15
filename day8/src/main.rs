use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let input: Vec<_> = include_str!("input.txt").split("\r\n").collect();
    let (h_grid, v_grid) = parse_input(input);
    let mut count = h_grid.len() * 4 - 4;
    let mut max_scene = 0;
    for i in 1..h_grid.len() - 1 {
        for j in 1..h_grid.len() - 1 {
            // Part 1
            let n = h_grid[i][j];
            let rows = h_grid[i][..j].iter().any(|x| *x >= n)
                && h_grid[i][j + 1..].iter().any(|x| *x >= n);

            let cols = v_grid[j][..i].iter().any(|x| *x >= n)
                && v_grid[j][i + 1..].iter().any(|x| *x >= n);

            if !(rows && cols) {
                count += 1;
            }
            // Part 2
            let left = std::cmp::min(
                h_grid[i][..j].iter().rev().take_while(|&x| *x < n).count() + 1,
                h_grid[i][..j].len(),
            );
            let right = std::cmp::min(
                h_grid[i][j + 1..].iter().take_while(|&x| *x < n).count() + 1,
                h_grid[i][j + 1..].len(),
            );
            let up = std::cmp::min(
                v_grid[j][..i].iter().rev().take_while(|&x| *x < n).count() + 1,
                v_grid[j][..i].len(),
            );
            let down = std::cmp::min(
                v_grid[j][i + 1..].iter().take_while(|&x| *x < n).count() + 1,
                v_grid[j][i + 1..].len(),
            );
            let scene = left * right * up * down;
            if scene > max_scene {
                max_scene = scene;
            }
        }
    }
    println!("count = {count}\nmax scenic value = {max_scene}");
    println!("Total time: {:?}", Instant::now().duration_since(t0));
}

fn parse_input(input: Vec<&str>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut h_grid = vec![];
    for line in input {
        let digits: Vec<_> = line.split("").collect();
        let digits: Vec<_> = digits
            .iter()
            .filter(|&x| *x != "")
            .map(|&x| x.chars().next().unwrap())
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        h_grid.push(digits);
    }
    let mut v_grid = vec![];
    for j in 0..h_grid[0].len() {
        let mut temp = vec![];
        for i in 0..h_grid.len() {
            temp.push(h_grid[i][j]);
        }
        v_grid.push(temp);
    }
    (h_grid, v_grid)
}
