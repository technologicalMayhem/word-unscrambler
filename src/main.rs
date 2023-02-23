#![feature(test)]

extern crate test;

use data::Trie;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::io;

mod data;

static DICT: Lazy<Trie> = Lazy::new(|| {
    let words: Vec<String> = include_str!("german.dic")
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
        .collect();

    let mut trie = Trie::new();
    for word in words {
        trie.insert(&word);
    }

    trie
});

struct Possibilities {
    full_matches: Vec<String>,
    all_combinations: Vec<String>,
    compound_matches: Vec<String>,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.replace('\n', "");

        if buffer.contains("quit") {
            break;
        }

        let possibilities = get_possibilities(&buffer);

        if !possibilities.full_matches.is_empty() {
            println!("Volltreffer:");
            for full_match in possibilities.full_matches {
                println!("{full_match}");
            }
        } else if !possibilities.compound_matches.is_empty() {
            println!("Folgende Möglichkeiten gefunden:");
            for compound_match in possibilities.compound_matches {
                println!("{compound_match}");
            }
        } else if !possibilities.all_combinations.is_empty() {
            println!("Keine Übereinstimmung gefunden. Zeige alle Kombinationen:");
            for combi in possibilities.all_combinations {
                println!("{combi}");
            }
        } else {
            println!("Nichts gefunden!");
        }
        println!();
    }
    Ok(())
}

fn get_possibilities(s: &str) -> Possibilities {
    let word_pieces: Vec<&str> = s.split(' ').collect();
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
        if DICT.search(&full) {
            full_matches.push(full);
        } else if check_match(&full) {
            compound_matches.push(full);
        } else {
            all_combinations.push(full);
        }
    }

    full_matches.dedup();
    all_combinations.dedup();
    compound_matches.dedup();

    Possibilities {
        full_matches,
        all_combinations,
        compound_matches,
    }
}

fn check_match(input: &str) -> bool {
    let mut chars = input.chars().peekable();
    let mut buffer = String::new();

    while let Some(c) = chars.next() {
        buffer.push(c);

        let result = DICT.search(&buffer);
        if !result {
            continue;
        }
        if chars.peek().is_some() {
            let s: String = chars.clone().collect();
            if DICT.search(&s) {
                return true;
            }
        } else {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        let cases = vec!["kaf sse ta fee", "hte n ac rb ah", "efg ge ara ti"];

        b.iter(|| {
            // Use `test::black_box` to prevent compiler optimizations from disregarding
            // Unused values

            for ele in &cases {
                get_possibilities(ele);
            }
        });
    }
}
