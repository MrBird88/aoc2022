use std::collections::VecDeque;

// Part 1
fn main() {
    let input = include_str!("input.txt").split("\r\n");
    let mut cargo = vec![VecDeque::new(); 9];

    for line in input.take_while(|x| x.contains("[")) {
        let line = line.replace("    ", " [ ]");
        let mut line: Vec<String> = line.split("] [").map(|x| x.to_string()).collect();

        line.iter_mut().enumerate().for_each(|(idx, chr)| {
            *chr = chr
                .chars()
                .filter(|x| x.is_alphabetic() || x.is_whitespace())
                .collect();

            if chr != " " {
                cargo[idx].push_front(chr.clone());
            }
        });
    }

    let input = include_str!("input.txt").split("\r\n");

    for line in input.skip_while(|x| !x.contains("move")) {
        let line: Vec<_> = line.split(" ").collect();
        let line: Vec<_> = line
            .iter()
            .filter_map(|x| usize::from_str_radix(x, 10).ok())
            .collect();

        for _ in 0..line[0] {
            let push = cargo[line[1] - 1].pop_back().unwrap();
            cargo[line[2] - 1].push_back(push);
        }
    }

    let mut answer = String::new();

    for vec in cargo {
        let letter = vec.iter().last().unwrap();
        answer.push(letter.chars().last().unwrap());
    }

    println!("{answer}");
}

// Part 2
// fn main() {
//     let input = include_str!("input.txt").split("\r\n");
//     let mut cargo: Vec<_> = vec![VecDeque::new(); 9];

//     for line in input.take_while(|x| x.contains("[")) {
//         let line = line.replace("    ", " [ ]");
//         let mut line: Vec<String> = line.split("] [").map(|x| x.to_string()).collect();

//         line.iter_mut().enumerate().for_each(|(idx, chr)| {
//             *chr = chr
//                 .chars()
//                 .filter(|x| x.is_alphabetic() || x.is_whitespace())
//                 .collect();

//             if chr != " " {
//                 cargo[idx].push_front(chr.clone());
//             }
//         });
//     }

//     let input = include_str!("input.txt").split("\r\n");

//     for line in input.skip_while(|x| !x.contains("move")) {
//         let mut line: Vec<_> = line.split(" ").collect();

//         let line: Vec<_> = line
//             .iter()
//             .filter_map(|x| usize::from_str_radix(x, 10).ok())
//             .collect();

//         let mut temp = VecDeque::new();

//         for _ in 0..line[0] {
//             let push = cargo[line[1] - 1].pop_back().unwrap();
//             temp.push_front(push);
//         }

//         while let Some(pop) = temp.pop_front() {
//             cargo[line[2] - 1].push_back(pop);
//         }
//     }

//     let mut answer = String::new();

//     for vec in cargo {
//         let letter = vec.iter().last().unwrap();
//         answer.push(letter.chars().last().unwrap());
//     }

//     println!("{answer}");
// }