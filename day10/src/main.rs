fn main() {
    let input: Vec<_> = include_str!("input.txt").split_whitespace().collect();
    let mut crt: Vec<_> = vec![vec!['.'; 40];6];
    let mut sig_strength: Vec<_> = vec![];
    let mut register: i64 = 1;
    let mut cycle = 0;
    for i in input {
        cycle += 1;
        let sprite = vec![register - 1, register, register + 1];
        let col = cycle % 40;
        let row = match cycle {
              1..=40  => 0,
             41..=80  => 1,
             81..=120 => 2,
            121..=160 => 3,
            161..=200 => 4,
            201..=240 => 5,
            _ => unreachable!(),
        };
        if sprite.contains(&((cycle - 1) % 40)) {
            crt[row][col as usize] = '#';
        }
        if (cycle - 20) % 40 == 0 && cycle <= 220 {
            let sig = cycle * register;
            sig_strength.push(sig);
        }
        if let Ok(val) = i.parse::<i64>() {
            register += val;
        }
    }
    let sum: i64 = sig_strength.iter().sum();
    println!("Total signal strength: {sum}");
    println!("CRT Screen Feed:");
    crt.iter().for_each(|vec| {
        let s: String = vec.iter().collect();
        println!("{}", s);
    });
}
