#![allow(unused_must_use)]
use evalexpr::*;
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let (mut monkey_items, monkey_ops, monkey_mods) = parse_input();
    let m_num = monkey_items.len();
    let mut monkey_count = vec![0; m_num];

    // Part 1
    // for _ in 0..20 {
    //     for m in 0..m_num {
    //         for idx in 0..monkey_items[m].len() {
    //             let mut context = HashMapContext::new();
    //             context.set_value("old".to_string(), monkey_items[m][idx].into());
    //             monkey_items[m][idx] = eval_int_with_context(&monkey_ops[m], &context)
    //                 .ok()
    //                 .unwrap()
    //                 / 3;
    //         }

    //         for _ in 0..monkey_items[m].len() {
    //             let val = monkey_items[m].pop_front().unwrap();
    //             match val % monkey_mods[m].0 == 0 {
    //                 true => {
    //                     monkey_items[monkey_mods[m].1].push_back(val);
    //                 }

    //                 false => {
    //                     monkey_items[monkey_mods[m].2].push_back(val);
    //                 }
    //             }
    //             monkey_count[m] += 1;
    //         }
    //     }
    // }

    // Part 2
    for _ in 0..10_000 {
        for m in 0..m_num {
            for idx in 0..monkey_items[m].len() {
                let mut context = HashMapContext::new();
                context.set_value("old".to_string(), monkey_items[m][idx].into());
                monkey_items[m][idx] = eval_int_with_context(&monkey_ops[m], &context)
                    .ok()
                    .unwrap()
                    % 9_699_690; // Used an online calculator... I'm terrible and I fell terrible.
            }

            for _ in 0..monkey_items[m].len() {
                let val = monkey_items[m].pop_front().unwrap();
                match val % monkey_mods[m].0 == 0 {
                    true => {
                        monkey_items[monkey_mods[m].1].push_back(val);
                    }

                    false => {
                        monkey_items[monkey_mods[m].2].push_back(val);
                    }
                }
                monkey_count[m] += 1;
            }
        }
    }

    let mut ans: i128 = 1;
    let mut vec = monkey_count.clone();
    vec.sort_unstable_by(|a, b| b.cmp(&a));
    vec.iter()
        .take(2)
        .map(|x| *x as i128)
        .for_each(|x| ans *= x);

    println!("Answer: {ans}.");

    println!("Total Time: {:?}", Instant::now().duration_since(t0));
}

fn parse_input() -> (Vec<VecDeque<i64>>, Vec<String>, Vec<(i64, usize, usize)>) {
    let mut input: Vec<_> = include_str!("input.txt").split("\r\n").collect();
    input.iter_mut().for_each(|x| *x = x.trim());

    let total_monkey_items = input.iter().filter(|&x| x.contains("Monkey")).count();
    let mut monkey_items: Vec<VecDeque<i64>> = vec![VecDeque::new(); total_monkey_items];
    let mut monkey_ops = Vec::with_capacity(total_monkey_items);
    let mut monkey_mod: Vec<_> = Vec::with_capacity(total_monkey_items);

    let mut instructions = vec![];
    let mut temp = Vec::with_capacity(6);
    for line in input {
        if line.is_empty() {
            instructions.push(temp.clone());
            temp.clear();
        } else {
            temp.push(line)
        }
    }
    instructions.push(temp);

    for i in &instructions {
        let m: Vec<_> = i[0]
            .split(&[' ', ':'])
            .into_iter()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let m = m[0];

        let items: VecDeque<i64> = i[1]
            .split(&[' ', ','])
            .into_iter()
            .filter(|x| x.parse::<i64>().is_ok())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        monkey_items[m] = items;

        let operation = i[2].replace("Operation: new = ", "");
        monkey_ops.push(operation.clone());

        let m_mod: Vec<_> = i[3]
            .split(" ")
            .into_iter()
            .filter(|x| x.parse::<i64>().is_ok())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let m_mod = m_mod[0];

        let m_true: Vec<_> = i[4]
            .split(&[' ', ':'])
            .into_iter()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let m_true = m_true[0];

        let m_false: Vec<_> = i[5]
            .split(&[' ', ':'])
            .into_iter()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let m_false = m_false[0];

        monkey_mod.push((m_mod, m_true, m_false));
    }
    (monkey_items, monkey_ops, monkey_mod)
}
