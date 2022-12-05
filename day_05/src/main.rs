use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
/*
            [C]         [N] [R]
[J] [T]     [H]         [P] [L]
[F] [S] [T] [B]         [M] [D]
[C] [L] [J] [Z] [S]     [L] [B]
[N] [Q] [G] [J] [J]     [F] [F] [R]
[D] [V] [B] [L] [B] [Q] [D] [M] [T]
[B] [Z] [Z] [T] [V] [S] [V] [S] [D]
[W] [P] [P] [D] [G] [P] [B] [P] [V]
 1   2   3   4   5   6   7   8   9

*/

fn main() {
    let mut file = File::open("day_05/input").unwrap();
    let buffer = BufReader::new(&mut file);
    let lines = buffer.lines().map(|l| l.unwrap());
    let mut stacks: Vec<String> = Vec::new();
    let mut code: Vec<String> = Vec::new();
    let mut vec_handle = &mut stacks;
    let mut columns: Vec<String> = vec![String::new(); 9];
    let mut mov: u32 = 0;
    let mut src: usize = 0;
    let mut dst: usize = 0;

    for line in lines {
        if line.trim().is_empty() {
            vec_handle = &mut code;
        } else {
            vec_handle.push(line);
        }
    }

    for line in stacks {
        let l: Vec<_> = line.chars().collect();
        for i in 0..l.len() {
            if i % 4 == 0 {
                if l[i] == '[' {
                    columns[i / 4].push(l[i + 1]);
                }
            }
        }
    }

    for i in 0..columns.len() {
        columns[i] = columns[i].chars().rev().collect::<String>()
    }

    let mut part_one = columns.clone();
    let mut part_two = columns.clone();

    for line in code {
        for (i, q) in line.split_ascii_whitespace().skip(1).step_by(2).enumerate() {
            match i {
                0 => mov = q.parse::<u32>().unwrap(),
                1 => src = q.parse::<usize>().unwrap() - 1,
                2 => dst = q.parse::<usize>().unwrap() - 1,
                _ => (),
            }
        }

        for _ in 0..mov {
            let c = part_one[src].pop().unwrap();
            part_one[dst].push(c);
        }

        let mut t: Vec<char> = Vec::new();
        for _ in 0..mov {
            t.push(part_two[src].pop().unwrap());
        }
        t.reverse();
        for c in t {
            part_two[dst].push(c)
        }
    }

    print!("Part One: ");
    for mut col in part_one {
        print!("{}", col.pop().unwrap());
    }
    println!();

    print!("Part Two: ");
    for mut col in part_two {
        print!("{}", col.pop().unwrap());
    }
    println!();
}
