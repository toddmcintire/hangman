use std::io::{self, stdout, Stdout, Write};
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
    let mut wrong_guesses = Vec::new();
    let mut correct_guesses = Vec::new();
    let mut stdout = stdout();
    let num_wrong_guesses = 6;
    let secret_phrase = get_request(&stdout).expect("TODO: panic message");

    clear_screen(&stdout);
    println!("Welcome to Hangman");
    println!("You have {} guesses to guess the word.", num_wrong_guesses);

    let secret_len = secret_phrase.chars().count() - 4;
    let mut display = Vec::new();

    for _ in 0..secret_len {
        display.push("_");
    }
    println!("{} {} {:?}", secret_phrase, secret_len, display);

    while display.contains(&"_") {
        // get input 
        println!("enter guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("no input");

        // check if guess is already used
        if wrong_guesses.contains(&guess) || correct_guesses.contains(&guess) {
            println!("you already guessed that, try again");
            continue;
        }

        // check if guess is in the secret word
        if secret_phrase.contains(&guess){
            correct_guesses.push(guess);
            // update display list with correct guess
            
        } else {
            wrong_guesses.push(guess);
            //print out stage
        }
    }
    
    //draw_hangman(&stdout);
    Ok(())
}





//how to move stuff
//out.queue(cursor::MoveTo(5, 5)).expect("TODO: panic message");
//out.flush().expect("error");
