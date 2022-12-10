use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let input: Vec<_> = input.chars().collect();
    let mut count = 13;
    for _ in input.iter().skip(13) {
        count += 1;
        let hash: HashSet<_> = input[count - 14..=count - 1].iter().collect();
        if hash.len() == 14 {
            break;
        }
    }
    println!("{count}")
}
