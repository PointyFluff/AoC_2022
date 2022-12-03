#![allow(unused, non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn score(letter: &char) -> u32 {
    let score: u32 = r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .position(|l| l == *letter)
        .unwrap() as u32;
    score + 1
}

fn main() -> std::io::Result<()> {
    /** part 1 */
    let mut file = File::open("day_03/input")?;
    let mut buffer = BufReader::new(&mut file);
    let lines = buffer.lines().map(|l| l.unwrap());
    let mut sum: u32 = 0;

    for line in lines {
        let l: Vec<_> = line.chars().collect();
        let (left, right) = l.split_at(l.len() / 2);
        for a in left {
            if right.contains(&a) {
                sum += score(&a);
                break;
            }
        }
    }
    println!("Part One: {sum}");

    /** part 2 **/
    let mut file = File::open("day_03/input")?;
    let mut buffer = BufReader::new(&mut file);
    let lines = buffer.lines().map(|l| l.unwrap());
    let mut sum: u32 = 0;
    let lines: Vec<_> = lines.collect();
    let len = lines.len();
    let group_size = 3;

    for i in 0..len {
        if i % group_size == 0 && i <= len - group_size {
            for letter in lines[i].chars() {
                if lines[i + 1].contains(letter) && lines[i + 2].contains(letter) {
                    sum += score(&letter);
                    break;
                }
            }
        }
    }
    println!("Part Two: {sum}");

    Ok(())
}
