#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    /** part 1 **/
    let mut file = File::open("day_02/input")?;
    let mut file = BufReader::new(file);
    let mut score: u32 = 0;

    for line in file.lines().map(|l| l.unwrap()) {
        let mut chars = line.trim().chars();
        // elfo chooses...
        match chars.next() {
            // rock...
            Some('A') => match chars.skip(1).next() {
                // rock +1
                Some('X') => score += 4, // draw +3
                // paper +2
                Some('Y') => score += 8, // win +6
                // scisors +3
                Some('Z') => score += 3, // loss +0
                Some(c) => println!("{c}"),
                None => (),
            },
            // paper...
            Some('B') => match chars.skip(1).next() {
                // rock +1
                Some('X') => score += 1, // loss +0
                // paper +2
                Some('Y') => score += 5, // draw +3
                // scisors +3
                Some('Z') => score += 9, // win +6
                Some(c) => println!("{c}"),
                None => (),
            },
            // scisors...
            Some('C') => match chars.skip(1).next() {
                // rock +1
                Some('X') => score += 7, // win +6
                // paper +2
                Some('Y') => score += 2, // loss +0
                // scisors +3
                Some('Z') => score += 6, // draw +3
                Some(c) => println!("{c}"),
                None => (),
            },
            Some(_) | None => println!("NOPE"),
        }
    }
    println!("Score: {score}");

    /** part 2 **/
    let mut file = File::open("day_02/input")?;
    let mut file = BufReader::new(file);
    let mut score: u32 = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let mut chars = line.trim().chars();
        // too lazy to enum, too tired to math...
        let scisors = 3;
        let paper = 2;
        let rock = 1;
        let win = 6;
        let draw = 3;
        let lose = 0;
        // elfo chooses...
        match chars.next() {
            // rock...
            Some('A') => match chars.skip(1).next() {
                // lose
                Some('X') => score += lose + scisors,
                // draw
                Some('Y') => score += draw + rock,
                // win
                Some('Z') => score += win + paper,
                Some(c) => println!("{c}"),
                None => (),
            },
            // paper...
            Some('B') => match chars.skip(1).next() {
                // lose
                Some('X') => score += lose + rock,
                // draw
                Some('Y') => score += draw + paper,
                // win
                Some('Z') => score += win + scisors,
                Some(c) => println!("{c}"),
                None => (),
            },
            // scisors...
            Some('C') => match chars.skip(1).next() {
                // lose
                Some('X') => score += lose + paper,
                // draw
                Some('Y') => score += draw + scisors,
                // win
                Some('Z') => score += win + rock,
                Some(c) => println!("{c}"),
                None => (),
            },
            Some(_) | None => println!("NOPE"),
        }
    }

    println!("Score: {score}");
    Ok(())
}
