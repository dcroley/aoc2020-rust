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

// fn process_line(hash: &mut HashMap<&str, String>) {
//
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    let mut hash:HashMap<String,isize> = HashMap::new();

    let mut sum = 0;
    let mut sum_all = 0;

    let mut group_len = 0;

    for aline in lines.iter() {

        if aline.is_empty() {
            // println!("group len = {}",hash.len());
            sum += hash.len();

            let mut all = 0;
            for &val in hash.values() {
                if val == group_len {
                    all += 1;
                }
            }
            println!("all answered = {}", all);
            sum_all += all;
            hash.clear();
            group_len = 0;
        } else {
            group_len += 1;
            for achar in aline.chars() {
                let mut cnt = 0;
                let tmp_key = String::from(achar);
                if hash.contains_key(&tmp_key) {
                    cnt = *hash.get(&tmp_key).unwrap();
                }
                hash.insert(tmp_key, cnt + 1);
            }
        }
    }

    // handle last one
    // println!("group = {}",hash.len());
    sum += hash.len();
    let mut all = 0;
    for &val in hash.values() {
        if val == group_len {
            all += 1;
        }
    }
    println!("all answered = {}", all);
    sum_all += all;

    println!("sum = {}", sum);
    println!("sum_all = {}", sum_all);
    Ok(())
}



