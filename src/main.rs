mod dictionary;
mod key;
mod parsing;
mod solver;

use std::env;
fn main() {
    // Input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong argument number, expected 1, got {}", args.len() - 1);
    }

    // Init
    let dict = dictionary::Dictionary::build();
    let puzzle = args.iter().nth(1).unwrap().to_ascii_uppercase();

    // Solve
    let key = solver::construct_key(&dict, puzzle.as_str());
    let translation = key.translate(puzzle.as_str());

    // Output
    println!("AXWDecipherSolver");
    println!("");
    println!("Constructed key: \n{}", key);
    println!("Words found in ciphertext:");
    for word in parsing::list_words(&puzzle) {
        let possible_words = dict.get_possible_words(word.as_str(), &key);
        println!("{:>20} | {:?}", word, possible_words);
    }
    println!();
    println!("Ciphertext:  {}", &puzzle);
    println!("Translation: {}", translation.unwrap());
}
