#[derive(Debug)]
struct State {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    prog: Vec<u8>,
    pc: usize,
    out: Vec<String>,
}

impl State {
    fn get_combo_operand(&self) -> usize {
        match self.prog[self.pc + 1] {
            x @ 0..=3 => x as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!(),
        }
    }
    fn exec_instruction(&mut self) {
        let literal_op = self.prog[self.pc + 1];
        match self.prog[self.pc] {
            0 => self.reg_a /= 2 << (self.get_combo_operand() - 1),
            1 => self.reg_b ^= literal_op as usize,
            2 => self.reg_b = self.get_combo_operand() % 8,
            3 => {
                if self.reg_a != 0 {
                    self.pc = literal_op as usize;
                    return;
                }
            }
            4 => self.reg_b ^= self.reg_c,
            5 => self.out.push((self.get_combo_operand() % 8).to_string()),
            6 => self.reg_b = self.reg_a / (2 << (self.get_combo_operand() - 1)),
            7 => self.reg_c = self.reg_a / (2 << (self.get_combo_operand() - 1)),
            _ => panic!("Invalid instruction"),
        }
        self.pc += 2;
    }
}

fn parse(input: &str) -> State {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let &[a, b, c] = registers
        .lines()
        .map(|l| l.split(' ').last().unwrap().parse().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    else {
        panic!()
    };
    State {
        reg_a: a,
        reg_b: b,
        reg_c: c,
        prog: program
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect(),
        pc: 0,
        out: vec![],
    }
}

fn part1(state: &mut State) -> String {
    while state.pc < state.prog.len() - 1 {
        state.exec_instruction();
    }
    state.out.join(",")
}

fn main() {
    let mut input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input));
}
