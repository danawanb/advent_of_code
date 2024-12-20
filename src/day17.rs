use std::{fs, usize};

struct Register {
    register_a: usize,
    register_b: usize,
    register_c: usize,
}

pub fn day_seventeen() -> usize {
    let prog = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 6, 5, 5, 3, 0];

    println!("{:?}", prog);
    let mut register: Register = Register {
        register_a: 51064159,
        register_b: 0,
        register_c: 0,
    };

    let exc = run_program(&prog, &mut register);
    println!("{}", exc.join(","));
    9
}

fn real_literal(operand: usize, reg: &Register) -> usize {
    match operand {
        0..=3 => operand,
        4 => reg.register_a,
        5 => reg.register_b,
        6 => reg.register_c,
        7 => panic!("Reserved operand: {}", operand),
        _ => unreachable!(),
    }
}
fn run_program(prog: &[usize], reg: &mut Register) -> Vec<String> {
    let mut pc = 0;
    let mut output = Vec::new();

    while pc + 1 < prog.len() {
        let opcode = prog[pc];
        let operand = prog[pc + 1];
        let combo = real_literal(operand, reg);

        match opcode {
            0 => {
                // adv
                let divisor = 2usize.pow(combo as u32);
                reg.register_a /= divisor;
            }
            1 => {
                // bxl
                reg.register_b ^= operand;
            }
            2 => {
                // bst
                reg.register_b = combo % 8;
            }
            3 => {
                // jnz
                if reg.register_a != 0 {
                    pc = operand;
                    continue;
                }
            }
            4 => {
                // bxc
                reg.register_b ^= reg.register_c;
            }
            5 => {
                // out
                output.push((combo % 8).to_string());
            }
            6 => {
                // bdv
                reg.register_b = reg.register_a / 2usize.pow(combo as u32);
            }
            7 => {
                // cdv
                reg.register_c = reg.register_a / 2usize.pow(combo as u32);
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }

        pc += 2; // Advance to next instruction
    }

    output
}
