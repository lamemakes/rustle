use std::fs;

use serde_json::Value;

use super::*;
use std::collections::HashSet;

#[test]
fn test_create_new_letter() {
    const TEST_LETTER: char = 'a';
    let new_letter = Letter::new(TEST_LETTER.clone(), LetterState::Correct);

    assert_eq!(new_letter.value, TEST_LETTER);
    assert_eq!(new_letter.status, LetterState::Correct)
}

#[test]
fn test_get_letter_color() {
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


#[test]
fn test_create_offline_wordlewords() -> Result<(), Box<dyn Error>> {
    const WORDLIST_PATH: &str = "src/assets/wordlist.json";

    let raw_wordlist = fs::read_to_string(&WORDLIST_PATH)?;
    let json_wordlist: Value = serde_json::from_str(&raw_wordlist)?;

    let wordlist: &Vec<Value> = json_wordlist.get("wordlist").expect("Did not find attribute \"wordlist\" in JSON file").as_array().expect("Failed to retrieve array from wordlist");

    // Convert &Vec<Value> to Vec<String> via mapping so the actual WordleWords.wordlist can be compared.
    let wordlist: Vec<String> = wordlist.iter().map(|e| e.as_str().expect("Failed to convert json value to str").to_string()).collect();

    let wordle_words = words::WordleWords::new(false)?;

    // Confirm the solkution exists within the worlist
    assert!(wordlist.contains(&wordle_words.solution.to_string()));

    let test_hash: HashSet<String> = wordlist.into_iter().collect();
    let wordle_words_hash: HashSet<String> = wordle_words.wordlist.into_iter().collect();
    let sym_diff: Vec<&String> = test_hash.symmetric_difference(&wordle_words_hash).collect();

    assert_eq!(sym_diff.len(), 0);

    Ok(())

}

// #[test]
// fn test_get_valid_user_input() {
//     let valid_guess = "cargo"
// }