pub mod display;
pub mod words;

use display::TermFormatter;

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

#[cfg(test)]
mod lib_tests;