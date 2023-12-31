use std::{str, env, process, io};
use rustle::display::{TermFormatter, RustleDisplay};
use rustle::words::WordleWords;
use rustle::{Letter, LetterState, get_user_guess, process_guess};

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();
    let wordle_words = WordleWords::new(args.len() > 1 && args[1].to_lowercase() == "--offline").unwrap_or_else(|err| {
        println!("Failed to initialize words: {}", err.to_string());
        process::exit(1)
    });

    let wordle_solution: &str = &wordle_words.get_solution().clone();

    let stdin = io::stdin();

    const MAX_TRIES: u8 = 6;

    let mut guess_list: [Vec<Letter>; 6] = [
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)], 
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)],
        vec![Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists), Letter::new(' ', LetterState::NotExists)]
    ];

    let mut rustle_display = match RustleDisplay::initialize_ui(wordle_words.is_offline()) {
        Ok(res) => res,
        Err(e) => panic!("Failed to initialize display: {}", e.to_string())
    };

    rustle_display.draw_logo().unwrap_or_else(|err| {
        panic!("Failed to draw logo: {}", err.to_string())
    });

    rustle_display.draw_ui(&guess_list).unwrap_or_else(|err| {
        panic!("Failed to draw logo: {}", err.to_string())
    });

    let mut guess = get_user_guess(&mut stdin.lock(), &mut rustle_display, &wordle_words).unwrap_or_else(|err| {
        panic!("Failed to draw logo: {}", err.to_string())
    });

    for attempt in 1..=MAX_TRIES {
        process_guess(&guess, &mut guess_list, wordle_solution, attempt);

        rustle_display.draw_ui(&guess_list).unwrap_or_else(|err| {
            panic!("Failed to draw logo: {}", err.to_string())
        });

        if guess == wordle_solution {
            println!(
                "{}WINNER!{} Word was \"{}{}{}\"", 
                TermFormatter::GreenBold.as_str(),
                TermFormatter::Clear.as_str(),
                TermFormatter::DefaultBold.as_str(),
                wordle_solution.to_uppercase(),
                TermFormatter::Clear.as_str()
            );

            rustle_display.terminate_ui().unwrap_or_else(|err| {
                panic!("Failed to draw logo: {}", err.to_string())
            });

            return
        } else if attempt == MAX_TRIES {
            println!(
                "Failed to guess in {} tries! Word was \"{}{}{}\"",
                MAX_TRIES,
                TermFormatter::DefaultBold.as_str(),
                wordle_solution.to_uppercase(),
                TermFormatter::Clear.as_str()
            );

            rustle_display.terminate_ui().unwrap_or_else(|err| {
                panic!("Failed to draw logo: {}", err.to_string())
            });

            return
        } else {
            guess = get_user_guess(&mut stdin.lock(), &mut rustle_display, &wordle_words).unwrap_or_else(|err| {
                panic!("Failed to draw logo: {}", err.to_string())
            });
        }
    }

}

#[cfg(test)]
mod bin_tests;