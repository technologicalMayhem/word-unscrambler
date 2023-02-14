use itertools::Itertools;
use once_cell::sync::Lazy;
use std::io;

static DICT: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("german.dic")
        .to_uppercase()
        .split('\n')
        .filter_map(|str| {
            let x: String = str.into();
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        })
        .collect()
});

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.replace('\n', "");

        if buffer.contains("quit") {
            break;
        }

        let word_pieces: Vec<&str> = buffer.split(' ').collect();
        let len = word_pieces.len();
        let permutations: Vec<Vec<&str>> = word_pieces.into_iter().permutations(len).collect();
        let mut full_matches: Vec<String> = Vec::new();
        let mut all_combinations: Vec<String> = Vec::new();
        let mut compound_matches: Vec<String> = Vec::new();

        for possibility in permutations {
            let mut full = String::new();
            for piece in possibility {
                full += piece;
            }
            let full = full.to_uppercase();
            if DICT.contains(&full) {
                full_matches.push(full);
            }
            else if check_match(&full) {
                compound_matches.push(full);
            } else {
                all_combinations.push(full);
            }
        }

            if !full_matches.is_empty() {
                println!("Volltreffer:");
                for full_match in full_matches {
                    println!("{full_match}");
                }
            }
            else if !compound_matches.is_empty() {
                println!("Folgende Möglichkeiten gefunden:");
                for compound_match in compound_matches {
                    println!("{compound_match}");
                }
            } else if !all_combinations.is_empty() {
                println!("Keine Übereinstimmung gefunden. Zeige alle Kombinationen:");
                for combi in all_combinations {
                    println!("{combi}");
                }
            } else {
                println!("Nichts gefunden!");
            }
        println!();
    }
    Ok(())
}

fn check_match(input: &str) -> bool {
    for ele in DICT.iter() {
        if input.starts_with(ele) {
            let reduce: String = input.chars().skip(ele.chars().count()).collect();
            if reduce.is_empty() {
                return true;
            }
            if check_match(&reduce) {
                return true;
            }
        }
    }

    false
}
