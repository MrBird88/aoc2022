use std::collections::HashSet;

fn main() {
    let mut count = 0;
    let input = include_str!("input.txt").split("\r\n");
    // let vec: Vec<_> = input.take(5).collect();
    // println!("{:?}", vec)
    for line in input {
        let vec: Vec<&str> = line.split(",").collect();

        let (i, j) = (vec[0], vec[1]);
        let (i, j) = (
            i.split("-").collect::<Vec<_>>(),
            j.split("-").collect::<Vec<_>>(),
        );

        let (a, b): (u64, u64) = (i[0].parse().unwrap(), i[1].parse().unwrap());
        let (x, y): (u64, u64) = (j[0].parse().unwrap(), j[1].parse().unwrap());

        let elf1: HashSet<_> = (a..=b).collect();
        let elf2: HashSet<_> = (x..=y).collect();

        let overlap: HashSet<_> = elf1.intersection(&elf2).collect();

        if overlap.len() > 0 {
            count += 1;
        }
    }
    println!("{count}");
}
