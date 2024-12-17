#[derive(Debug, Clone)]
struct State {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    prog: Vec<u8>,
    pc: usize,
    out: Vec<u8>,
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
            0 => self.reg_a >>= self.get_combo_operand(),
            1 => self.reg_b ^= literal_op as usize,
            2 => self.reg_b = self.get_combo_operand() & 7,
            3 => {
                if self.reg_a != 0 {
                    self.pc = literal_op as usize;
                    return;
                }
            }
            4 => self.reg_b ^= self.reg_c,
            5 => self.out.push((self.get_combo_operand() & 7) as u8),
            6 => self.reg_b = self.reg_a >> self.get_combo_operand(),
            7 => self.reg_c = self.reg_a >> self.get_combo_operand(),
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

fn get_output(state: &mut State) -> Vec<u8> {
    while state.pc < state.prog.len() - 1 {
        state.exec_instruction();
    }
    state.out.clone()
}

fn part1(state: &mut State) -> String {
    get_output(state)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// B <- A % 8
// B <- B ^ 3
// C <- A >> B
// B <- B ^ C
// A <- A >> 3
// B <- B ^ 5
// print B & 7
// jmp 0 if A

fn find_input(state: &State, instructions: &[u8], curr_a: usize) -> Vec<usize> {
    let mut candidates = vec![];
    if instructions.is_empty() {
        return vec![curr_a];
    }
    let curr = instructions.last().unwrap();
    for i in 0..8 {
        let mut new_state = state.clone();
        let new_a = (curr_a << 3) + i;
        new_state.reg_a = new_a;
        if get_output(&mut new_state)[0] == *curr {
            candidates.push(new_a);
        }
    }
    let mut results = vec![];
    for candidate in candidates {
        results.extend(find_input(
            state,
            &instructions[0..instructions.len() - 1],
            candidate,
        ));
    }
    results
}

fn part2(state: &State) -> usize {
    *find_input(state, &state.prog, 0).iter().min().unwrap()
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("{}", part1(&mut input.clone()));
    println!("{}", part2(&input));
}
