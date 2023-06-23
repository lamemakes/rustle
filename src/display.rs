use std::{io, fmt::format};
use std::io::Write;
use std::boxed::Box;
use crossterm::{QueueableCommand, ExecutableCommand, cursor, terminal};
use crate::Letter;

pub enum TermFormatter {
    GreenBg,
    YellowBg,
    WhiteBg,
    GrayBg,
    GreenFg,
    RedFg,
    BlackFg,
    Clear,
    DefaultBold,
    BlackBold,
    GreenBold,
    RedBold,
    SlowBlink
}

impl TermFormatter {
    pub fn as_str(&self) -> String {
        match self {
            TermFormatter::GreenBg => String::from("\x1b[102m"),
            TermFormatter::YellowBg => String::from("\x1b[103m"),
            TermFormatter::WhiteBg => String::from("\x1b[47m"),
            TermFormatter::GrayBg => String::from("\x1b[100m"),
            TermFormatter::GreenFg => String::from("\x1b[0;92m"),
            TermFormatter::BlackFg => String::from("\x1b[0;30m"),
            TermFormatter::RedFg => String::from("\x1b[0;31m"),
            TermFormatter::DefaultBold => String::from("\x1b[1m"),
            TermFormatter::BlackBold => TermFormatter::get_bold(&TermFormatter::BlackFg).to_owned(),
            TermFormatter::GreenBold => TermFormatter::get_bold(&TermFormatter::GreenFg).to_owned(),
            TermFormatter::RedBold => TermFormatter::get_bold(&TermFormatter::RedFg).to_owned(),
            TermFormatter::Clear => String::from("\x1b[0m"),
            TermFormatter::SlowBlink => String::from("\x1b[5m")
        }
    }
    // This is neccesary as it appears Windows CLIs cannot handle stacked ANSI. ie. \x1b[1;30m would just be black
    fn get_bold<'a>(color: &'a TermFormatter) -> String {
        format!("{}{}", color.as_str(), TermFormatter::DefaultBold.as_str())
    }
}

pub struct Logo {}

impl Logo {
    pub fn get_logo(offline: bool) -> &'static str {
        const OFFLINE_STR: &str = "OFFLINE!";
        // Logo generated by:
        // https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Roman&text=Rustle
        // (tinkered with a bit)
        let logo: &str = 
        "
 ooooooooo.                          ooo   oooo            
 `888   `Y88.                        888   '888            
  888   .d88' oooo  oooo   .oooo.o  88888   888   .ooooo.  
  888ooo88P'  '888  '888  d88(  \"8  '888'   888  d88' '88b 
  888`^\\888b   888   888  `\"Y88b.    888    888  888ooo888 
 .888.  `888.  888   888. o.  )88b   888.  .888. 888. .ooo 
 o888o   888o  `V88V\"V888 8\"\"888P'   8888  88888  `Y88888P'\n";

        if offline {
            let mut spacer = String::new();
            for _ in 1..((60 - OFFLINE_STR.len()) / 2) { spacer.push(' ') }
            let format_logo = format!(
                "{}{}{}{}{}{}{}\n\n\n",
                logo,
                &spacer,
                TermFormatter::GreenFg.as_str(),
                TermFormatter::SlowBlink.as_str(),
                TermFormatter::DefaultBold.as_str(), 
                &OFFLINE_STR, 
                TermFormatter::Clear.as_str()
            ).to_owned();

            Box::leak(format_logo.into_boxed_str())
        } else {
            Box::leak(format!("{}\n\n", &logo).into_boxed_str())
        }
    }
}

pub struct RustleDisplay {
    stdout: io::Stdout,
    overall_height: u8,
    overall_width: u8,
    game_height: u8, // The height of the main game canvas, not including the logo.
    offline: bool
}

impl RustleDisplay {
    pub fn initialize_ui(offline: bool) -> RustleDisplay {
        // Initialize a "canvas" that accounts for the size of the logo, input fields, and actual game UI.

        let mut stdout: io::Stdout = io::stdout();

        stdout.queue(cursor::SavePosition).unwrap();

        const GAME_HEIGHT: u8 = 6 * 2 + 2;  // The size of the guess_list array with a space after each item, with two lines for prompts & inputs.

        let overall_height = usize::from(GAME_HEIGHT) + Logo::get_logo(offline).lines().count();

        for _ in 1..=(overall_height) {
            stdout.write_all(format!("\n").as_bytes()).unwrap();
        }
    
        RustleDisplay { 
            stdout: stdout,
            overall_height: u8::try_from(overall_height).expect("Failed to convert logo line length to u8!"),
            overall_width: 58,
            game_height: 6 * 2 + 2,
            offline: offline
        }
    }

    pub fn draw_logo(&mut self) {
        self.stdout.queue(cursor::MoveUp(u16::from(self.overall_height))).unwrap();
        self.stdout.write_all(format!("{}{}{}", TermFormatter::DefaultBold.as_str(), Logo::get_logo(self.offline), TermFormatter::Clear.as_str()).as_bytes()).unwrap();
        self.stdout.queue(cursor::MoveDown(u16::from(self.game_height))).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn draw_ui(&mut self, guess_list: &[Vec<Letter>; 6]) {
        self.stdout.queue(cursor::MoveUp(u16::from(self.game_height))).unwrap();
        self.stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    
        for guess in guess_list {
            for _ in 1..=((self.overall_width - 20)/2) {
                self.stdout.write_all(" ".as_bytes()).unwrap();
            }
            for letter in guess {
                self.stdout.write_all(format!("{}{} {} {} ", TermFormatter::BlackBold.as_str(), letter.get_ansi_color(), letter.value, TermFormatter::Clear.as_str()).as_bytes()).unwrap();
            }
            self.stdout.write_all("\n\n".as_bytes()).unwrap();
        }
        self.stdout.flush().unwrap();
    
        self.stdout.execute(cursor::Show).unwrap();
 
    }

    pub fn draw_input_error(&mut self, error_msg: &str) {
        self.stdout.queue(cursor::MoveUp(cursor::position().expect("Failed to get cursor position").0 + 2)).unwrap();
        self.stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        self.stdout.write_all(format!("{}{}{}", TermFormatter::RedFg.as_str(), error_msg, TermFormatter::Clear.as_str()).as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn terminate_ui(&mut self) {
        self.stdout.queue(cursor::MoveUp(u16::from(self.overall_height))).unwrap();
        for _ in 1..=(self.overall_height) {
            self.stdout.write_all(format!("\n").as_bytes()).unwrap();
        }
        self.stdout.queue(cursor::RestorePosition).unwrap();
    }
}