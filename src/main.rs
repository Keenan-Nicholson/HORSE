#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]

extern crate csv;
extern crate rand;

use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Words.csv";

    let file: File = File::open(file_path)?;
    let mut reader: csv::Reader<File> = csv::Reader::from_reader(file);

    let mut word_bank: Vec<String> = Vec::new();
    for record in reader.records() {
        let record: csv::StringRecord = record?;
        let word: String = record.get(0).ok_or("Invalid CSV format")?.to_string();
        word_bank.push(word);
    }

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..word_bank.len());

    let random_word: String = word_bank[random_index].clone();

    let blank_word: String = "_".repeat(random_word.len());
    let mut chars: Vec<char> = blank_word.chars().collect(); // Convert string to mutable character array

    let mut correct: i32 = 0;
    let mut incorrect: i32 = 0;

    let horse: &str = "HORSE";

    while correct != random_word.len().try_into().unwrap() {
        let temp: i32 = correct;

        let mut guess: String = String::new();

        println!("Please enter a letter: ");
        io::stdin().read_line(&mut guess).expect("failed to read");

        let trimmed_guess: &str = guess.trim();
        let lowercase_guess = trimmed_guess.to_lowercase();

        for (index, c) in random_word.chars().enumerate() {
            if c.to_string() == lowercase_guess {
                println!("CORRECT GUESS!");
                if let Some(element) = chars.get_mut(index) {
                    *element = c;
                }
                let string_so_far: String = chars.iter().collect();
                println!("{}", string_so_far);
                correct += 1;
                if correct == random_word.len().try_into().unwrap() {
                    println!("YOU GOT IT, THE WORD WAS {}", random_word)
                }
            }
        }
        if temp == correct {
            let substring = &horse[..=incorrect as usize];
            println!("INCORRECT GUESS: {}", substring);

            if substring == horse {
                println!("GAME OVER!");
                println!("THE WORD WAS {}", random_word);
                break;
            };
            incorrect += 1;
        }
    }

    Ok(())
}
