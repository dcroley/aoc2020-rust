
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    let mut exec_cnt = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        exec_cnt.insert(i, 0);
    }

    let mut curr_line = 0;
    let mut acc = 0;

    while 1 == 1 {
        if curr_line == lines.len() {
            println!("Normal End of Execution, curr_line={}, acc={}", curr_line, acc);
            break;
        }
        if exec_cnt[curr_line] == 1 {
            println!("DUPE EXEC FOUND, curr_line={}, acc={}", curr_line, acc);
            break;
        }
        let aline = lines.get(curr_line).unwrap();
        exec_cnt[curr_line] = 1;
        let (instruction, operand) = aline.split_at(3);
        if instruction == "nop" {
            println!("{} nop", curr_line);
            curr_line += 1;
            continue;
        } else if instruction == "acc" {
            // println!("acc {}", operand.trim());
            let val = operand.trim().parse::<isize>().unwrap();
            acc = acc + val;
            println!("{} acc {}, acc={}", curr_line, val, acc);
            curr_line += 1;
        } else if instruction == "jmp" {
            let val = operand.trim().parse::<isize>().unwrap();
            println!("{} jmp {}, acc={}", curr_line, val, acc);
            curr_line = (curr_line as isize + val) as usize;
        }
    }

    // part 2

    let mut normal_exit = false;
    let mut instruction_to_swap = 0;

    while !normal_exit {
        // find a jmp or nop to switch
        while !(lines.get(instruction_to_swap).unwrap().contains("nop") ||
            lines.get(instruction_to_swap).unwrap().contains("jmp")) {
            instruction_to_swap += 1;
            println!("instruction to swap = {}", instruction_to_swap);
        }
        let mut curr_line = 0;
        let mut acc = 0;
        // reset exec cnts
        for i in 0..lines.len() {
            exec_cnt[i] = 0;
        }

        while 1 == 1 {
            if curr_line == lines.len() {
                println!("Normal End of Execution, curr_line={}, acc={}", curr_line, acc);
                normal_exit = true;
                break;
            }
            if exec_cnt[curr_line] == 1 {
                // println!("DUPE EXEC FOUND, curr_line={}, acc={}", curr_line, acc);
                break;
            }
            let aline = lines.get(curr_line).unwrap();
            exec_cnt[curr_line] = 1;
            let (instruction, operand) = aline.split_at(3);
            if curr_line == instruction_to_swap {
                if instruction == "nop" {
                    let val = operand.trim().parse::<isize>().unwrap();
                    println!("{} jmp {}, acc={}", curr_line, val, acc);
                    curr_line = (curr_line as isize + val) as usize;
                }
                if instruction == "jmp" {
                    println!("{} nop", curr_line);
                    curr_line += 1;
                    continue;
                }
            } else if instruction == "nop" {
                println!("{} nop", curr_line);
                curr_line += 1;
                continue;
            } else if instruction == "acc" {
                let val = operand.trim().parse::<isize>().unwrap();
                acc = acc + val;
                println!("{} acc {}, acc={}", curr_line, val, acc);
                curr_line += 1;
            } else if instruction == "jmp" {
                let val = operand.trim().parse::<isize>().unwrap();
                println!("{} jmp {}, acc={}", curr_line, val, acc);
                curr_line = (curr_line as isize + val) as usize;
            }
        }
        instruction_to_swap += 1;
    }

    Ok(())
}



