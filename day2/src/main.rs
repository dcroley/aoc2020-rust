use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

struct PasswdLine {
    rule: String,
    chr: String,
    passwd: String,
}

fn build_passwd_line(aline: String) -> PasswdLine {
    PasswdLine {
        rule: aline.split(" ").nth(0).unwrap().to_string(),
        chr: aline.split(" ").nth(1).unwrap().replace(":", "").to_string(),
        passwd: aline.split(" ").nth(2).unwrap().to_string(),
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<PasswdLine> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|line| build_passwd_line(line))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    // part 1
    let mut cnt_good_part1 = 0;
    for aline in lines.iter() {
        let cnt = aline.passwd.chars().filter(|c| c.to_string() == aline.chr).count();
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let caps = re.captures(aline.rule.as_str()).unwrap();
        if caps.get(1).unwrap().as_str().parse::<u32>().unwrap() <= cnt.to_string().parse::<u32>().unwrap() &&
            cnt.to_string().parse::<u32>().unwrap() <= caps.get(2).unwrap().as_str().parse::<u32>().unwrap() {
            println!("good: {} {} {}", aline.rule, aline.chr, aline.passwd);
            cnt_good_part1 += 1;
        }
    }
    println!("part 1 good cnt={}", cnt_good_part1);

    // part2
    let mut cnt_good_part2 = 0;
    for aline in lines.iter() {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let caps = re.captures(aline.rule.as_str()).unwrap();
        let index1 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let index2 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let a = aline.passwd.chars().nth(index1);
        let b =  aline.passwd.chars().nth(index2);
        if a.is_none() || b.is_none() {
            continue;
        }
        if (a.unwrap().to_string() == aline.chr || b.unwrap().to_string() == aline.chr) &&
            !(a.unwrap().to_string() == aline.chr && b.unwrap().to_string() == aline.chr) {
            // println!("good: {} {} {}", aline.rule, aline.chr, aline.passwd);
            cnt_good_part2 += 1;
        } else {
            // println!("bad: {} {} {}", aline.rule, aline.chr, aline.passwd);
        }
    }
    println!("part 2 good cnt={}", cnt_good_part2);


    Ok(())
}

