use text_io::read;
use std::{
    fs::File,
    io::{
        self,
        BufRead
    },
    path::Path
};
use rand::seq::SliceRandom;
use ansi_term::Colour::{Red, Green, Blue};

fn main() {
    let mut guessed_letters = Vec::new();
    let word = get_random_word();
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
        println!("{}", Red.paint("No guesses left!"));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_random_word() -> String {
    let mut rng = rand::thread_rng();

    if let Ok(lines) = read_lines("./words.txt") {
        // Consumes the iterator, returns an (Optional) String
        let words: Vec<String> = lines.map(|x| x.unwrap()).collect();
        return words.choose(&mut rng).unwrap().to_string()
    }
    String::from("default")
}

fn get_letter(guesses_left: &usize) -> String {
    println!("Enter letter ({} guesses left):", guesses_left);
    read!()
}

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