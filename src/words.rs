use std::error::Error;

use chrono::Datelike;
use serde::Deserialize;
use rand::seq::SliceRandom;
use crate::display::TermFormatter;

#[derive(Deserialize)]
struct SolutionResponse {
    solution: String
}

#[derive(Deserialize)]
struct WordList {
    wordlist: Vec<String>
}

pub struct WordleWords {
    solution: String,
    wordlist: Vec<String>,
    offline: bool
}

impl WordleWords {
    pub fn new(offline: bool) -> Result<WordleWords, Box<dyn Error>> {
        let mut offline = offline;
        let wordlist = WordleWords::load_wordlist()?;

        let solution: String;

        if !offline {
            let remote_solution = WordleWords::get_remote_solution();

            solution = match remote_solution {
                Ok(sol) => sol,
                Err(err) => {
                    println!(
                        "{}{} A random solution will be used.{}",
                        TermFormatter::RedBold.as_str(),
                        err,
                        TermFormatter::Clear.as_str()
                    );
                    offline = true;
                    WordleWords::get_random_local_solution(&wordlist)?
                }
            }
        } else {
            solution = WordleWords::get_random_local_solution(&wordlist)?;
        }

        Ok(WordleWords { solution: solution, wordlist: wordlist, offline: offline})
    }

    fn get_remote_solution() -> Result<String, &'static str> {
        let day: String = if chrono::Local::now().date_naive().day() < 10 {
            format!("0{}", chrono::Local::now().date_naive().day())
        } else {
            chrono::Local::now().date_naive().day().to_string()
        };

        let month: String = if chrono::Local::now().date_naive().month() < 10 {
            format!("0{}", chrono::Local::now().date_naive().month())
        } else {
            chrono::Local::now().date_naive().month().to_string()
        };

        let date_str = format!("{}-{}-{}", chrono::Local::now().date_naive().year(), month, day);
        let nyt_wordlist_url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", date_str);

        let word_req = reqwest::blocking::get(nyt_wordlist_url);
        
        let word_json: Result<SolutionResponse, reqwest::Error> = match word_req {
            Ok(json_res) => json_res.json(),
            Err(_) => return Err("Failed to retrieve the remote solution.")
        };

        match word_json {
            Ok(res) => return Ok(res.solution),
            Err(_) => return Err("Failed to parse remote soltuion JSON.")
        }
    }

    fn get_random_local_solution(wordlist: &Vec<String>) -> Result<String, &str> {
        match wordlist.choose(&mut rand::thread_rng()) {
            Some(rand_solution) => Ok(rand_solution.to_owned()),
            None => Err("Failed to retrieve a new local word")
        }
    }

    fn load_wordlist() -> Result<Vec<String>, Box<dyn Error>> {
        let raw_wordlist = include_str!("assets/wordlist.json");

        let wordlist: WordList = serde_json::from_str(raw_wordlist)?;

        Ok(wordlist.wordlist)
    }

    pub fn is_offline(&self) -> bool {
        return self.offline
    }

    pub fn get_solution(&self) -> &String {
        return &self.solution
    }

    pub fn get_wordlist(&self) -> &Vec<String> {
        return &self.wordlist
    }

}