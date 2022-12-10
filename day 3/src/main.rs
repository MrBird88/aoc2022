#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").split("\r\n");
    let mut input = input.into_iter();
    let mut priority = 0;
    while let Some(v) = input.next() {
        let (a, b, c) = (
            v.chars().collect::<HashSet<_>>(),
            input.next().unwrap().chars().collect::<HashSet<_>>(),
            input.next().unwrap().chars().collect::<HashSet<_>>(),
        );
        let i1: HashSet<_> = b.intersection(&c).map(|x| *x).collect();
        let intersection = *a.intersection(&i1).into_iter().next().unwrap() as u8;
        let value = match intersection.gt(&96) {
            true => intersection - b'a' + 1,
            false => intersection - b'A' + 27,
        };
        priority += value as u64;
    }
    println!("{priority}");
}
