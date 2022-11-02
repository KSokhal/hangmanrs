use text_io::read;
use ansi_term::Colour::{Red, Green, Blue};
use serde::{Serialize, Deserialize};
use reqwest::Error;

#[derive(Deserialize, Serialize, Debug)]
struct Word {
    word: String,
}

fn main() {
    let word = get_random_word().expect("API request failed");

    let mut guessed_letters: Vec<char> = Vec::new();
    let mut guesses_left: usize = word.len() + 10;
    
    let mut correct = show_word(word.to_string(), &guessed_letters);
    
    while !correct {
        if guesses_left ==0 {
            break;
        }
        let mut i: String = get_letter(&guesses_left);
        while i.len() != 1 {
            println!("{}", Red.paint("Only one letter can be entered at a time"));
            i = get_letter(&guesses_left);
        }
        let k: Vec<char> = i.chars().collect();
        guessed_letters.push(k[0].to_ascii_lowercase());
        correct = show_word(word.to_string(), &guessed_letters);
        guesses_left -= 1;
    };
    if correct {
        println!("{}", Green.paint("Correct!"));
    } else {
        println!("{}", Red.paint(format!("No guesses left!\nThe word was: {}", word)));
    }
}

// Attempted to get a random word using a public API
fn get_random_word() -> Result<String, Error> {
    let word: Word = reqwest::blocking::get("https://random-word-api.herokuapp.com/word")?.json::<Word>()?;
    Ok(word.word)
}

// Asks the user for a letter and reads the reponse
fn get_letter(guesses_left: &usize) -> String {
    println!("Enter letter ({} guesses left):", guesses_left);
    read!()
}

// Displays the word with any unknown characters replaced with an underscore
// Returns true if there are no unknown characters
fn show_word(word: String, guesses_letters: &[char]) -> bool{
    let guessed_word: String = word.chars()
    .map(|x| {
        if guesses_letters.contains(&x.to_ascii_lowercase()) {
            x
        } else {
            String::from("_").chars().next().unwrap()
        }
    })
    .collect();
    println!("{}", Blue.paint(&guessed_word));
    if guessed_word.contains(&"_") {
        return false
    }
    true
}