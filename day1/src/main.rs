fn main() {
    let input = include_str!("input.txt").split("\r\n\r\n");
    let mut total_calories = vec![];
    for elf in input {
        let mut cal_count = 0;
        for snack in elf.lines() {
            let snack: u64 = snack.parse().unwrap();
            cal_count += snack;
        }
        total_calories.push(cal_count);
    }
    total_calories.sort_unstable_by(|a, b| b.cmp(a));
    println!(
        "The fattest elf has {} calories worth of snacks.",
        total_calories[..3].iter().sum::<u64>()
    );
}
