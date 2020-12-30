use std::fs;

#[derive(Clone)]
struct Register {
    ins: String,
    amt: i32,
    dir: char,
    visits: i32,
}

impl Register {
    pub fn new(ins: String, amt: i32, dir: char) -> Self {
        Self {
            ins,
            amt,
            dir,
            visits: 0,
        }
    }
}

pub fn execute_dayeight() {
    let path = "./input/day8.txt";
    let working = build_sequence(prepare_input(path));
    let (accumulator_after_one, visited_registers) = run_sequence_once(working.clone());
    println!(
        "On first pass, the accumulator was {}",
        accumulator_after_one
    );
    //Again, unless I want to change the return types to be a Vec the result won't be returned here
    //Maybe do this later
    let _modified_accumulator = modify_sequence(working, visited_registers);
}

fn modify_sequence(seq: Vec<Register>, modify: Vec<usize>) -> i32 {
    let nop = "nop";
    let jmp = "jmp";
    let terminated_accumulator = 0;
    let seq_copy = seq;
    let modify_copy = modify;

    for x in 0..modify_copy.len() {
        let mut modified_seg = seq_copy.clone();
        if modified_seg[modify_copy[x]].ins.as_str().eq(nop) {
            modified_seg[modify_copy[x]].ins = jmp.to_string();
        } else {
            modified_seg[modify_copy[x]].ins = nop.to_string();
        }
        //I can't not have the return but I don't have a good way to use them here
        let (_garbage, _other_garbage) = run_sequence_once(modified_seg);
    }
    terminated_accumulator
}

fn run_sequence_once(seq: Vec<Register>) -> (i32, Vec<usize>) {
    let mut track_registers: Vec<usize> = Vec::new();
    let p = '+';
    let m = '-';
    let acc = "acc";
    let nop = "nop";
    let jmp = "jmp";
    let breakout = seq.len();
    let mut accumulator = 0;
    let mut pointer: usize = 0;

    let mut copy_seq = seq;

    loop {
        if pointer >= breakout {
            println!(
                "Breaking out from pointer ({}) with {}",
                &pointer, &accumulator
            );
            break;
        }
        let work: Register = copy_seq[pointer].clone();
        if work.visits == 1 {
            break;
        } else {
            let get_instruction = work.ins.clone();
            if get_instruction.as_str().eq(acc) {
                copy_seq[pointer].visits += 1;
                if copy_seq[pointer].dir.eq(&p) || copy_seq[pointer].dir.eq(&m) {
                    accumulator += copy_seq[pointer].amt;
                }
                pointer += 1;
            } else if get_instruction.as_str().eq(nop) {
                copy_seq[pointer].visits += 1;
                track_registers.push(pointer);
                pointer += 1;
            } else if get_instruction.as_str().eq(jmp) {
                copy_seq[pointer].visits += 1;
                track_registers.push(pointer);
                let temp_pointer = pointer;
                if copy_seq[pointer].dir.eq(&p) {
                    pointer += copy_seq[temp_pointer].amt as usize;
                } else if copy_seq[pointer].dir.eq(&m) {
                    pointer = (pointer as i32 + copy_seq[temp_pointer].amt) as usize;
                }
            }
        }
    }
    (accumulator, track_registers)
}

fn build_sequence(raw_sequence: Vec<String>) -> Vec<Register> {
    let mut ret: Vec<Register> = Vec::new();

    for x in raw_sequence {
        let (instruction, amount) = x.split_at(x.find(' ').unwrap());
        let dir: char = amount.trim().chars().next().unwrap();
        ret.push(Register::new(
            instruction.to_string(),
            amount.trim().parse::<i32>().unwrap(),
            dir,
        ));
    }
    ret
}

fn prepare_input(filepath: &str) -> Vec<String> {
    let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
    let ret = list
        .as_str()
        .split('\n')
        .map(str::parse::<String>)
        .map(Result::unwrap)
        .collect();
    ret
}
