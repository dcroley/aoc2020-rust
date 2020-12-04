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

    // part 1
    let mut forest = [['a'; 324]; 31];
    let mut x = 0;
    let mut y = 0;

    for aline in lines.iter() {
       for ch in aline.chars() {
           // println!("got ch={}, x={}, y={}", ch, x, y);
           forest[x][y] = ch;
           x += 1;
       }
        y += 1;
        x = 0;
    }
    x = 0;
    y = 0;
    let mut trees = 0;

    while y < 322 {
        x += 3;
        if x > 30 {
            x = x - 31
        }
        y += 1;
        if forest[x][y] == '#' {
            trees += 1;
        }
    }

    // for a in 0..31 {
    //     print!("{}", forest[a][0]);
    // }
    // println!();

    println!("Part 1: number of trees={}", trees);

    // part2
    let slopes = [[1, 1], [3, 1], [5,1], [7,1], [1,2]];
    let mut total = 1;

    for t in 0..5 {
        trees = 0;
        x = 0;
        y = 0;
        println!("{} by {}", slopes[t][0], slopes[t][1]);
        while y <= 322 {
            if forest[x][y] == '#' {
                trees += 1;
            }
            x += slopes[t][0];
            if x > 30 {
                x = x - 31
            }
            y += slopes[t][1];
        }
        println!("trees for slope {} = {}", t, trees);
        total = total * trees;
    }

    println!("total = {}", total);

    Ok(())
}

