pub mod display;
pub mod words;

use std::error::Error;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

use display::{TermFormatter, RustleDisplay};

#[derive(Debug, PartialEq, Eq)]
pub enum LetterState {
    Exists,
    NotExists,
    Correct,
    Incorrect
}

pub struct Letter {
    value: char,
    status: LetterState
}

impl Letter {
    pub fn new(value: char, status: LetterState) -> Letter {
        Letter { value: value, status: status }
    }

    pub fn value(&self) -> char {
        self.value
    }

    pub fn status(&self) -> &LetterState {
        &self.status
    }

    pub fn get_ansi_color(&self) -> String {
        match self.status {
            LetterState::Correct => TermFormatter::GreenBg.as_str(),
            LetterState::Exists => TermFormatter::YellowBg.as_str(),
            LetterState::Incorrect => TermFormatter::GrayBg.as_str(),
            LetterState::NotExists => TermFormatter::WhiteBg.as_str()
        }
    }
}

pub fn get_user_guess(stdin: &io::Stdin, display_man: &mut RustleDisplay, wordle_words: &words::WordleWords) -> Result<String, Box<dyn Error>> {
    const WORD_GUESS_PROMPT: &str = "Enter a word guess:";

    let mut guess = String::new();

    println!("{}", WORD_GUESS_PROMPT);
    stdin.read_line(&mut guess)?;

    guess = guess.trim().to_string();

    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-zA-Z]{5}$").expect("Faied to create RegEx");
    }

    while !RE.is_match(&guess) || !wordle_words.wordlist.contains(&String::from(&guess)) {

        display_man.draw_input_error(format!("Invalid word \"{}\"! Please enter a new guess:\n", &guess).as_str())?;

        guess.clear();

        io::stdin().read_line(&mut guess)?;
        guess = guess.trim().to_string();
    }

    Ok(guess.to_lowercase())
}

#[cfg(test)]
mod lib_tests;