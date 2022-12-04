#![allow(unused, non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut file = File::open("day_04/input").unwrap();
    let mut buffer = BufReader::new(&mut file);
    let lines = buffer.lines().map(|l| l.unwrap());
    let mut sum_one: u32 = 0;
    let mut no_one: u32 = 0;

    for line in lines {
        let pair = line.split_once(',').unwrap();
        let left = pair.0.split_once('-').unwrap();
        let right = pair.1.split_once('-').unwrap();
        let l = (
            left.0.parse::<u32>().unwrap(),
            left.1.parse::<u32>().unwrap(),
        );
        let r = (
            right.0.parse::<u32>().unwrap(),
            right.1.parse::<u32>().unwrap(),
        );
        // had a really cool solution with ranges,
        // but that didn't behave as expected.
        // back to c-style.

        // does one set contain the other?
        if l.0 <= r.0 && l.1 >= r.1 || r.0 <= l.0 && r.1 >= l.1 {
            // part one
            sum_one += 1;
        }

        // does one set intersect the other?
        if l.0 <= r.1 && l.1 >= r.0 || r.0 <= l.1 && r.1 >= l.0 {
            // part two
            no_one += 1;
        }
    }

    println!("Part One: {sum_one}");
    println!("Part Two: {no_one}");
}
