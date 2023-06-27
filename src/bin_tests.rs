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