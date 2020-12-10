
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

static PREAMBLE:usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    let mut numbers:Vec<isize> = Vec::with_capacity(lines.len());

    for aline in lines {
        numbers.push(aline.parse().unwrap());
    }

    let mut magic_number = 0;

    for i in PREAMBLE..numbers.len() {
        let mut valid = false;
        for a in i-PREAMBLE..i-1 {
            for b in a+1..i {
                // println!("{} + {}", numbers[a], numbers[b]);
                if (numbers[a] + numbers[b] == numbers[i]) && (numbers[a] != numbers[b]) {
                    // println!("{} + {} = {}", numbers[a], numbers[b], numbers[i]);
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }
        if !valid {
            println!("{} is valid={}", numbers[i], valid);
            magic_number = numbers[i];
            break;
        }
    }

    // part 2
    let mut found_it = false;
    for a in 0..numbers.len()-1 {
        let mut sum = numbers[a];
        if sum > magic_number {
            continue;
        }
        for b in a+1..numbers.len() {
            sum += numbers[b];
            if sum == magic_number {
                println!("found it from {}, {}", numbers[a], numbers[b]);
                let mut smallest = numbers[a];
                let mut largest = numbers[b];
                for x in a..b+1 {
                    if numbers[x] < smallest {
                        smallest = numbers[x];
                    }
                    if numbers[x] > largest {
                        largest = numbers[x];
                    }
                }
                println!("{} + {} = {}", smallest, largest, smallest + largest);
                found_it = true;
                break;
            }
            if sum > magic_number {
                break;
            }
        }
        if found_it {
            break;
        }
    }

    Ok(())
}



