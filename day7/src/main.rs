
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;
use std::collections::HashMap;


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn search_for_bag(rules:&HashMap<String,Vec<&str>>, search_point:&String, bag:&str, depth:isize) -> bool {
    let mut found = false;
    // println!("{} at {} looking for {}", depth, search_point, bag);
    if search_point == &bag.to_string() {
        // println!("found itself");
        return true;
    }
    for b in rules.get(search_point).unwrap() {
        // println!("  {}", b.to_string());
        let (cnt, bag_name) = b.split_at(1);
        found = search_for_bag(rules, &bag_name.trim().to_string(), bag, depth + 1);
        if found {
            break;
        }
    }
    // println!("found = {}", found);
    found
}

fn count_bags(rules:&HashMap<String,Vec<&str>>, search_point:&str) -> isize {
    let mut count = 1;
    println!("counting in {}", search_point);

    for b in rules.get(search_point).unwrap() {
        let (cnt, bag_name) = b.split_at(1);
        println!("  {}:{}", bag_name.to_string(), cnt);
        count = count + cnt.parse::<isize>().unwrap() * count_bags(rules, &bag_name.trim().to_string());
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = lines_from_file("input.txt");

    let mut rules:HashMap<String,Vec<&str>> = HashMap::new();

    let line_regex = Regex::new(r"(.*) bags contain (.*)\.").unwrap();

    for aline in lines.iter() {
        let caps = line_regex.captures(aline.as_str()).unwrap();
        let key = caps.get(1).unwrap().as_str();
        print!("key bag = {}: ", key);
        let mut v:Vec<&str> = Vec::new();
        for b in caps.get(2).unwrap().as_str().split(", ") {
            print!("bag={} ", b);
            if b != "no other bags" {
                let bag_name_regex = Regex::new(r"(\d+ .*) bags*").unwrap();
                let bag_name = bag_name_regex.captures(b).unwrap().get(1).unwrap().as_str();
                v.push(bag_name);
            }
        }
        println!();
        rules.insert(key.to_string(), v);
    }

    // println!("entry={}", rules.get("shiny gold").unwrap().len());

    let mut total = 0;

    for abag in rules.keys() {
        if search_for_bag(&rules, abag, "shiny gold", 0) {
            println!("outer bag = {}", abag);
            total += 1;
        }
    }

    // substract 1 to account for itself. Can't contain yourself
    println!("total = {}", total-1);

    // part 2
    let bags_in_bag = count_bags(&rules, "shiny gold") - 1;
    println!("bags in bag={}", bags_in_bag);

    Ok(())
}



