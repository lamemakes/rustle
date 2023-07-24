pub mod display;
pub mod words;

use std::{error::Error, io::BufRead};
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

pub fn get_user_guess<R>(stdin: &mut R, display_man: &mut RustleDisplay, wordle_words: &words::WordleWords) -> Result<String, Box<dyn Error>>
    where
        R: BufRead
    {
    const WORD_GUESS_PROMPT: &str = "Enter a word guess:";

    let mut guess = String::new();

    println!("{}", WORD_GUESS_PROMPT);
    stdin.read_line(&mut guess)?;

    guess = guess.trim().to_string();

    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-zA-Z]{5}$").expect("Faied to create RegEx");
    }

    while !RE.is_match(&guess) || !wordle_words.get_wordlist().contains(&String::from(&guess)) {

        display_man.draw_input_error(format!("Invalid word \"{}\"! Please enter a new guess:\n", &guess).as_str())?;

        guess.clear();

        stdin.read_line(&mut guess)?;
        guess = guess.trim().to_string();
    }

    Ok(guess.to_lowercase())
}

pub fn process_guess(user_guess: &str, guess_list: &mut [Vec<Letter>; 6], solution: &str, attempt: u8) {

    let user_guess_chars: Vec<char> = user_guess.chars().collect();
    let solution_chars: Vec<char> = solution.chars().collect();

    let mut current_guess: Vec<Letter> = Vec::new();


    for (index, char) in user_guess_chars.iter().enumerate() {
        let status: LetterState;
        if char == &solution_chars[index] {
            status = LetterState::Correct;
        } else if solution_chars.contains(char) {
            status = LetterState::Exists;
        } else {
            status = LetterState::Incorrect;
        }

        current_guess.push(Letter::new(*char, status))
    }

    guess_list[usize::from(attempt)-1] = current_guess;
}

#[cfg(test)]
mod lib_tests;