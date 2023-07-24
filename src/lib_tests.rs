use std::fs;

use serde_json::Value;

use super::*;
use std::collections::HashSet;

#[test]
fn create_new_letter() {
    const TEST_LETTER: char = 'a';
    let new_letter = Letter::new(TEST_LETTER.clone(), LetterState::Correct);

    assert_eq!(new_letter.value, TEST_LETTER);
    assert_eq!(new_letter.status, LetterState::Correct)
}

#[test]
fn get_letter_color() {
    const TEST_LETTER: char = 'a';

    let new_letter_one = Letter::new(TEST_LETTER.clone(), LetterState::Correct);

    let new_letter_two = Letter::new(TEST_LETTER.clone(), LetterState::Incorrect);

    let new_letter_three = Letter::new(TEST_LETTER.clone(), LetterState::Exists);

    let new_letter_four = Letter::new(TEST_LETTER.clone(), LetterState::NotExists);

    assert_eq!(new_letter_one.get_ansi_color(), TermFormatter::GreenBg.as_str());
    assert_eq!(new_letter_two.get_ansi_color(), TermFormatter::GrayBg.as_str());
    assert_eq!(new_letter_three.get_ansi_color(), TermFormatter::YellowBg.as_str());
    assert_eq!(new_letter_four.get_ansi_color(), TermFormatter::WhiteBg.as_str());
}


struct TestWordList {
    wordlist: Vec<String>
}

impl TestWordList {
    fn new() -> Result<TestWordList, Box<dyn Error>> {
        let raw_wordlist = fs::read_to_string("src/assets/wordlist.json")?;
        let json_wordlist: Value = serde_json::from_str(&raw_wordlist)?;

        let wordlist: &Vec<Value> = json_wordlist.get("wordlist").expect("Did not find attribute \"wordlist\" in JSON file").as_array().expect("Failed to retrieve array from wordlist");

        // Convert &Vec<Value> to Vec<String> via mapping so the actual WordleWords.wordlist can be compared.
        let wordlist: Vec<String> = wordlist.iter().map(|e| e.as_str().expect("Failed to convert json value to str").to_string()).collect();
 
        Ok(TestWordList{ wordlist: wordlist })
    }
}

#[test]
fn create_offline_wordlist() -> Result<(), Box<dyn Error>> {
    let wordle_words = words::WordleWords::new(false)?;
    let test_word_list = TestWordList::new()?;

    let test_hash: HashSet<String> = test_word_list.wordlist.into_iter().collect();
    let wordle_words_hash: HashSet<String> = wordle_words.get_wordlist().clone().into_iter().collect();
    let sym_diff: Vec<&String> = test_hash.symmetric_difference(&wordle_words_hash).collect();

    assert_eq!(sym_diff.len(), 0);

    Ok(())
}

#[test]
fn create_offline_solution() -> Result<(), Box<dyn Error>> {
    let wordle_words = words::WordleWords::new(false)?;
    let test_word_list = TestWordList::new()?;

    assert!(test_word_list.wordlist.contains(&String::from(wordle_words.get_solution())));

    Ok(())
}

// TODO: Online word solution tests! Mock reqwest!

#[test]
fn process_guess_list_population() {
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