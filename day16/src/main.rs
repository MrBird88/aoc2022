#![allow(dead_code)]

fn main() {
    let (flow_rates, connections) = parse_input();
    let combine: Vec<_> = flow_rates.iter().zip(connections).collect();
    for (line, vec) in combine {
        println!("Valve {} has a flow rate of {}", line.0, line.1);
        println!("It is connected to room(s):");
        for room in vec {
            println!("{}", room);
        }
    }
}

fn parse_input() -> (Vec<(String, u32)>, Vec<Vec<String>>) {
    let input: Vec<_> = include_str!("../input.txt").split("\r\n").collect();

    let mut connections = vec![];
    let mut flow_rates = vec![];

    for line in input {
        let vec: Vec<_> = line.split(&[' ', ';', ',', '=']).collect();
        let mut temp_connect = vec![];
        let mut temp_flow = (" ".to_string(), 0);

        for (idx, s) in vec
            .iter()
            .filter(|&x| (x.len() == 2 && *x != "to") || x.parse::<u32>().is_ok())
            .map(|x| *x)
            .enumerate()
        {
            match idx {
                0 => {
                    temp_flow.0 = s.to_string();
                }
                1 => {
                    temp_flow.1 = s.parse::<u32>().unwrap();
                }
                _ => {
                    temp_connect.push(s.to_string());
                }
            }
        }

        connections.push(temp_connect);
        flow_rates.push(temp_flow);
    }

    (flow_rates, connections)
}
