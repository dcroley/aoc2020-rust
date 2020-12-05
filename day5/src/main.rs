use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::collections::HashMap;


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn lower_half(start: usize, size:usize) -> (usize, usize) {
    (start, size / 2)
}

fn upper_half(start:usize, size:usize) -> (usize, usize) {
    (start + size / 2, size / 2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    let mut highest_seat = 0;

    let mut plane = [['a'; 8]; 121];

    let mut seats = HashMap::new();

    // part 1
    for aline in lines.iter() {
        let mut start_row = 0;
        let mut size_row = 128;
        for i in 0..7 {
            let c:char = aline.as_bytes()[i] as char;
            if c == 'B' {
                let tmp = upper_half(start_row, size_row);
                start_row = tmp.0;
                size_row = tmp.1;
            } else if c == 'F' {
                let tmp = lower_half(start_row, size_row);
                start_row = tmp.0;
                size_row = tmp.1;
            }
            // println!("start={}, size={}", start_row, size_row);
        }

        let mut start_col = 0;
        let mut size_col = 8;
        for i in 7..10 {
            let c:char = aline.as_bytes()[i] as char;
            if c == 'R' {
                let tmp = upper_half(start_col, size_col);
                start_col = tmp.0;
                size_col = tmp.1;
            } else if c == 'L' {
                let tmp = lower_half(start_col, size_col);
                start_col = tmp.0;
                size_col = tmp.1;
            }
            // println!("start={}, size={}", start_col, size_col);
        }
        let seat = start_row * 8 + start_col;
        println!("row = {}, col = {}, seat = {}",start_row, start_col, seat);
        plane[start_row][start_col] = 'X';
        seats.insert(seat, 1);
        if seat > highest_seat {
            highest_seat = seat;
        }
    }
    println!("highest seat={}", highest_seat);

    // part2

    for x in 0..121 {
        for y in 0..8 {
            if plane[x][y] != 'X' {
                let tmp = x * 8 + y;
                if tmp == 0 {
                    continue;
                }
                let plus = tmp + 1;
                let minus = tmp - 1;
                if seats.contains_key(&plus) && seats.contains_key(&minus) {
                    println!("Found your seat! row={}, col={}, seat={}, plus={}, minus={}", x, y, tmp,
                             seats.contains_key(&plus), seats.contains_key(&minus));
                }
            }
        }
    }

    Ok(())
}



