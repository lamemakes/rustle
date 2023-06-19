use std::{io, str, env};
use lazy_static::lazy_static;
use regex::Regex;

mod display;
use display::{TermFormatter, RustleDisplay};

mod words;
use words::WordleWords;

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
    pub fn get_ansi_color(&self) -> &'static str{
        match self.status {
            LetterState::Correct => TermFormatter::GreenBg.as_str(),
            LetterState::Exists => TermFormatter::YellowBg.as_str(),
            LetterState::Incorrect => TermFormatter::GrayBg.as_str(),
            LetterState::NotExists => TermFormatter::WhiteBg.as_str()
        }
    }
}

fn get_user_guess(display_man: &mut RustleDisplay, wordle_words: &WordleWords) -> String {
    const WORD_GUESS_PROMPT: &str = "Enter a word guess:";

    let mut guess = String::new();

    println!("{}", WORD_GUESS_PROMPT);
    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    guess = guess.trim().to_string();

    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-zA-Z]{5}$").unwrap();
    }

    while !RE.is_match(&guess) || !wordle_words.wordlist.contains(&String::from(&guess)) {

        display_man.draw_input_error(format!("Invalid word \"{}\"! Please enter a new guess:\n", &guess).as_str());

        guess.clear();

        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        guess = guess.trim().to_string();
    }

    guess.to_lowercase()
}

fn process_guess(user_guess: &str, guess_list: &mut [Vec<Letter>; 6], solution: &str, attempt: u8) {

    let user_guess_chars: Vec<char> = user_guess.chars().collect();
    let solution_chars: Vec<char> = solution.chars().collect();

    let mut current_guess: Vec<Letter> = Vec::new();


    for (index, char) in user_guess_chars.iter().enumerate() {
        if char == &solution_chars[index] {
            current_guess.push(Letter { value: *char, status: LetterState::Correct })
        } else if solution_chars.contains(char) {
            current_guess.push(Letter { value: *char, status: LetterState::Exists })
        } else {
            current_guess.push(Letter { value: *char, status: LetterState::Incorrect })
        }
    }

    guess_list[usize::from(attempt)-1] = current_guess;
}

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();
    let wordle_words = WordleWords::initialize(args.len() > 1 && args[1] == "--offline");

    let wordle_solution: &str = &wordle_words.solution.to_owned();

    const MAX_TRIES: u8 = 6;

    let mut guess_list: [Vec<Letter>; 6] = [
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }], 
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }],
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }],
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }],
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }],
        vec![Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }, Letter { value: ' ', status: LetterState::NotExists }]
    ];

    let mut rustle_display = RustleDisplay::initialize_ui(wordle_words.offline);
    rustle_display.draw_logo();

    rustle_display.draw_ui(&guess_list);

    let mut guess = get_user_guess(&mut rustle_display, &wordle_words);

    for attempt in 1..=MAX_TRIES {
        process_guess(&guess, &mut guess_list, wordle_solution, attempt);
        rustle_display.draw_ui(&guess_list);

        if guess == wordle_solution {
            println!("{}{}WINNER!{} Word was \"{}{}{}\"", 
            TermFormatter::GreenFg.as_str(),
            TermFormatter::Bold.as_str(),
            TermFormatter::Clear.as_str(),
            TermFormatter::Bold.as_str(),
            wordle_solution.to_uppercase(),
            TermFormatter::Clear.as_str()
            
        );
            rustle_display.terminate_ui();
            return
        } else if attempt == MAX_TRIES {
            println!("Failed to guess in {} tries! Word was \"{}\"", MAX_TRIES, wordle_solution);
            rustle_display.terminate_ui();
            return
        } else {
            guess = get_user_guess(&mut rustle_display, &wordle_words);
        }
    }

}

