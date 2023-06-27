use super::*;

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