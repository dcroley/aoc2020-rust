use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input1.txt");
    for x in 1..lines.len() - 1 {
        for y in x + 1..lines.len() {
            if lines[x].parse::<u32>().unwrap() + lines[y].parse::<u32>().unwrap() == 2020 {
                println!("Part 1: Found it x={}, y={}, product={}", x, y, lines[x].parse::<u32>().unwrap() * lines[y].parse::<u32>().unwrap());
            }
        }
    }
    for x in 1..lines.len() - 2 {
        for y in x + 1..lines.len() - 1 {
            for z in y + 1 .. lines.len() {
                if lines[x].parse::<u32>().unwrap() + lines[y].parse::<u32>().unwrap() + lines[z].parse::<u32>().unwrap() == 2020 {
                    println!("Part 2: Found it x={}, y={}, z={}, product={}", x, y, z,
                             lines[x].parse::<u32>().unwrap() * lines[y].parse::<u32>().unwrap() * lines[z].parse::<u32>().unwrap());
                }
            }
        }
    }

    Ok(())
}

