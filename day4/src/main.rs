use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashMap;
use regex::Regex;

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
    let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut passport:HashMap<&str,String> = HashMap::new();

    let mut total_present = 0;
    let mut total_valid = 0;

    for aline in lines.iter() {

        if aline.is_empty() {
            let mut valid_keys = true;
            let mut valid_values = true;
            for k in req.iter() {
                if !passport.contains_key(k) {
                    valid_keys = false;
                    break;
                } else {
                    let mut value_valid = true;
                    if k.to_string() == "byr" {
                        let year:u16 = passport.get(k).unwrap().parse().unwrap();
                        if year < 1920 || year > 2002 {
                            value_valid = false;
                        }
                    } else if k.to_string() == "iyr" {
                        let year:u16 = passport.get(k).unwrap().parse().unwrap();
                        if year < 2010 || year > 2020 {
                            value_valid = false;
                        }
                    } else if k.to_string() == "eyr" {
                        let year:u16 = passport.get(k).unwrap().parse().unwrap();
                        if year < 2020 || year > 2030 {
                            value_valid = false;
                        }
                    } else if k.to_string() == "hgt" {
                        let val = passport.get(k).unwrap();
                        if val.ends_with("cm") {
                            let hgt:u16 = val.strip_suffix("cm").unwrap().parse().unwrap();
                            if hgt < 150 || hgt > 193 {
                                value_valid = false;
                            }
                        } else if val.ends_with("in") {
                            let hgt:u16 = val.strip_suffix("in").unwrap().parse().unwrap();
                            if hgt < 59 || hgt > 76 {
                                value_valid = false;
                            }
                        } else {
                            value_valid = false;
                        }
                    } else if k.to_string() == "hcl" {
                        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                        if !re.is_match(passport.get(k).unwrap()) {
                            value_valid = false;
                        }
                    } else if k.to_string() == "ecl" {
                        let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                        if !re.is_match(passport.get(k).unwrap()) {
                            value_valid = false;
                        }
                    } else if k.to_string() == "pid" {
                        let re = Regex::new(r"^\d{9}$").unwrap();
                        if !re.is_match(passport.get(k).unwrap()) {
                            value_valid = false;
                        }
                    }
                    println!("k={}, v={}, valid={}", k.to_string(), passport.get(k).unwrap(), value_valid);
                    valid_values &= value_valid;
                }
            }
            if valid_keys {
                total_present += 1;
            }
            if valid_values && valid_keys {
                total_valid += 1;
            }
            passport.clear();

        } else {

            for kp in aline.split_whitespace() {
                let key = kp.split(":").nth(0);
                let val = kp.split(":").nth(1);
                passport.insert(key.unwrap(), val.unwrap().to_string());
            }

        }
    }

    // handle last one
    // let mut valid = true;
    // for k in req.iter() {
    //     if !passport.contains_key(k) {
    //         valid = false;
    //         break;
    //     }
    // }
    // if valid {
    //     total_present += 1;
    // }
    // passport.clear();

    println!("total = {}", total_present);

    // part2

    println!("total valid = {}", total_valid);

    Ok(())
}

