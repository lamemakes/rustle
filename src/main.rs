use std::{io, str, env};
use lazy_static::lazy_static;
use regex::Regex;
use rustle::display::{TermFormatter, RustleDisplay};
use rustle::words::WordleWords;
use rustle::{Letter, LetterState};

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

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();
    let wordle_words = WordleWords::initialize(args.len() > 1 && args[1] == "--offline");

    let wordle_solution: &str = &wordle_words.solution.to_owned();

    const MAX_TRIES: u8 = 6;

    let mut guess_list: [Vec<Letter>; 6] = [
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)], 
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)]
    ];

    let mut rustle_display = RustleDisplay::initialize_ui(wordle_words.offline);
    match rustle_display.draw_logo() {
        Err(e) => panic!("Failed to draw logo: {}", e.to_string()),
        Ok(_) => {}
    }

    match rustle_display.draw_ui(&guess_list) {
        Err(e) => panic!("Failed to draw logo: {}", e.to_string()),
        Ok(_) => {}
    }

    let mut guess = get_user_guess(&mut rustle_display, &wordle_words);

    for attempt in 1..=MAX_TRIES {
        process_guess(&guess, &mut guess_list, wordle_solution, attempt);
        match rustle_display.draw_ui(&guess_list) {
            Err(e) => panic!("Failed to draw logo: {}", e.to_string()),
            Ok(_) => {}
        }

        if guess == wordle_solution {
            println!(
                "{}WINNER!{} Word was \"{}{}{}\"", 
                TermFormatter::GreenBold.as_str(),
                TermFormatter::Clear.as_str(),
                TermFormatter::DefaultBold.as_str(),
                wordle_solution.to_uppercase(),
                TermFormatter::Clear.as_str()
            );

            rustle_display.terminate_ui();
            return
        } else if attempt == MAX_TRIES {
            println!(
                "Failed to guess in {} tries! Word was \"{}{}{}\"",
                MAX_TRIES,
                TermFormatter::DefaultBold.as_str(),
                wordle_solution.to_uppercase(),
                TermFormatter::Clear.as_str()
            );
            rustle_display.terminate_ui();
            return
        } else {
            guess = get_user_guess(&mut rustle_display, &wordle_words);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_populates_guess_list() {
        let user_guess = "nouns";

        let mut guess_list: [Vec<Letter>; 6] = [
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)], 
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
            vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)]
        ];

        let solution = "snaps";
        let solution_chars: Vec<char> = solution.chars().collect();
        let attempt = 3;
        
        process_guess(user_guess, &mut guess_list, solution, attempt);

        for (letter, char) in guess_list[usize::from(attempt) - 1].iter().zip(user_guess.chars()) {
            assert_eq!(letter.value(), char);
        }

        for (letter, char) in guess_list[usize::from(attempt) - 1].iter().zip(solution.chars()) {
            let status: LetterState;
            if letter.value() == char {
                status = LetterState::Correct;
            } else if solution_chars.contains(&letter.value()) {
                status = LetterState::Exists;
            } else {
                status = LetterState::Incorrect;
            }

            assert_eq!(letter.status(), &status);
        }
    }
}