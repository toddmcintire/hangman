use std::io::{stdout, Stdout, Write};
use std::error::Error;

use crossterm::{cursor, QueueableCommand, terminal::Clear};
use crossterm::terminal::ClearType;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn clear_screen(mut out: &Stdout){
    out.queue(Clear(ClearType::All)).expect("TODO: panic message");
    out.flush().expect("error");
}

fn get_request(mut out: &Stdout) -> Result<String, Box<dyn Error>>{
    let resp = reqwest::blocking::get("https://random-word-api.herokuapp.com/word")?.text()?;
    Ok(resp)
}

fn main() -> std::io::Result<()> {
    let mut stages:[&str; 7] = ["0", "1", "2", "3", "4", "5", "6"];
    //let wrong_guesses = Vec::new();
    //let correct_guesses = Vec::new();
    let mut stdout = stdout();
    let num_wrong_guesses = 6;
    let secret_phrase = get_request(&stdout).expect("TODO: panic message");

    clear_screen(&stdout);
    println!("Welcome to Hangman");
    println!("You have {} guesses to guess the word.", num_wrong_guesses);

    let secret_len = secret_phrase.chars().count() - 4;
    let mut display = Vec::new();

    for i in 0..secret_len {
        display.push("_");
    }
    println!("{} {} {:?}", secret_phrase, secret_len, display);
    //draw_hangman(&stdout);
    Ok(())
}





//how to move stuff
//out.queue(cursor::MoveTo(5, 5)).expect("TODO: panic message");
//out.flush().expect("error");
