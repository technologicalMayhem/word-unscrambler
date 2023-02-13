use itertools::Itertools;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let file = include_str!("german.dic").to_uppercase();
    let dict: Vec<&str> = file.split('\n').collect();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.replace("\n", "");

        if buffer.contains("quit") {
            break;
        }

        let word_pieces: Vec<&str> = buffer.split(' ').collect();
        let len = word_pieces.len();
        let permutations: Vec<Vec<&str>> = word_pieces.into_iter().permutations(len).collect();
        let mut combinations: Vec<String> = Vec::new();        
        let mut found_valid = false;

        for possibility in permutations {
            let mut full = String::new();
            for piece in possibility {
                full += piece;
            }
            let full = full.to_uppercase();
            if dict.contains(&full.as_str()) {
              println!("{full}");
              found_valid = true;
            }
            combinations.push(full);
        }
         
        if ! found_valid && ! combinations.is_empty() {
        println!("Keine Übereinstimmung gefunden. Zeige alle Kombinationen:");
        for combi in combinations {
          println!("{combi}");
        }}
    }
    Ok(())
}
