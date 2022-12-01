#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut file = File::open("day_01/input")?;
    let mut file = BufReader::new(file);
    let mut calories: Vec<u32> = vec![0];
    let mut elf: usize = 0;

    for line in file.lines().map(|l| l.unwrap()) {
        if !line.trim().is_empty() {
            calories[elf] += line.parse::<u32>().unwrap();
        } else {
            calories.push(0);
            elf += 1;
        }
    }

    calories.sort();
    calories.reverse();
    println!("{:?}", &calories[0..3]);
    println!("Most Calories: {}", calories[0..3].iter().sum::<u32>());
    Ok(())
}
