// Puzzle link : https://www.codingame.com/ide/puzzle/kangaroo-words

use std::{io, collections::HashMap};

use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut kangaroos: HashMap<String, Vec<String>> = HashMap::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let lines = input_line.trim_matches('\n').to_string();

        let words = lines.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();

        words.iter().for_each(|kangaroo| {
            words.iter().filter(|&joey| {
                if joey == kangaroo { 
                    return false;
                }

                let mut index_kangaroo = 0;
                for ch in joey.chars() {
                    if let Some(index) = kangaroo[index_kangaroo..].find(ch) {
                        index_kangaroo += index + 1;
                    } else {
                        return false;
                    }
                }
                true
            }).for_each(|joey| {
                kangaroos.entry(kangaroo.to_string())
                    .and_modify(|joeys| joeys.push(joey.to_string()))
                    .or_insert_with(|| vec![joey.to_string()]);
            });
        });
    }

    if kangaroos.is_empty() {
        println!("NONE");
    } else {
        kangaroos.iter()
        .sorted_by(|(k1,_), (k2, _)| k1.cmp(k2))
        .for_each(|(k, j)| {
            println!("{}: {}", k, j.join(", "));
        });
    }
}

