use std::{cmp::Reverse, collections::HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Gate {
    InputGate(bool),
    CalcGate(CalcGate),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CalcGate {
    operation: Operation,
    lhs: String,
    rhs: String,
}

fn parse(input: &str) -> HashMap<String, Gate> {
    let mut res = HashMap::new();
    let (inputs, calcs) = input.split_once("\n\n").unwrap();
    res.extend(inputs.lines().map(|l| {
        let (name, val) = l.split_once(": ").unwrap();
        (name.to_string(), Gate::InputGate(val == "1"))
    }));
    res.extend(calcs.lines().map(|l| {
        let mut iter = l.split(' ');
        let lhs = iter.next().unwrap();
        let op = iter.next().unwrap();
        let rhs = iter.next().unwrap();
        let name = iter.nth(1).unwrap();
        (
            name.to_string(),
            Gate::CalcGate(CalcGate {
                operation: Operation::from(op),
                lhs: lhs.to_string(),
                rhs: rhs.to_string(),
            }),
        )
    }));
    res
}

fn get_gate_value(
    gate: &str,
    gates: &HashMap<String, Gate>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(v) = cache.get(gate) {
        return *v;
    }
    let res = match gates.get(gate).unwrap() {
        Gate::InputGate(val) => *val,
        Gate::CalcGate(calc_gate) => match calc_gate.operation {
            Operation::And => {
                get_gate_value(&calc_gate.lhs, gates, cache)
                    & get_gate_value(&calc_gate.rhs, gates, cache)
            }
            Operation::Or => {
                get_gate_value(&calc_gate.lhs, gates, cache)
                    | get_gate_value(&calc_gate.rhs, gates, cache)
            }
            Operation::Xor => {
                get_gate_value(&calc_gate.lhs, gates, cache)
                    ^ get_gate_value(&calc_gate.rhs, gates, cache)
            }
        },
    };
    cache.insert(gate.to_string(), res);
    res
}

fn part1(gates: &HashMap<String, Gate>) -> usize {
    let mut output_bits: Vec<_> = gates.keys().filter(|g| g.starts_with("z")).collect();
    let mut cache = HashMap::new();
    output_bits.sort_unstable_by_key(|v| Reverse(v.to_string()));
    let mut res: usize = 0;
    for bit in output_bits {
        res <<= 1;
        if get_gate_value(bit, gates, &mut cache) {
            res += 1;
        }
    }
    res
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&input));
}
