use std::io;
use std::process::Command;

fn main() {
    clear_screen();

    println!("Put in a word to guess!");

    let mut word_to_guess = String::new();

    io::stdin()
        .read_line(&mut word_to_guess)
        .expect("Failed to read input");

    word_to_guess = word_to_guess.to_lowercase().trim().to_string();

    clear_screen();

    println!("Guess a character!");

    let mut current_game_state = String::new();
    let mut found_chars_counter = 0;

    for _ in 0..word_to_guess.len() {
        current_game_state.push('_');
    }

    println!("Current game state: {}", current_game_state);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let mut guess: char = match guess.trim().parse() {
            Ok(single_char) => single_char,
            Err(_) => continue,
        };

        if guess.is_alphabetic() {
            println!("You guessed: {}", guess);

            guess = guess
                .to_lowercase()
                .to_string()
                .chars()
                .next()
                .expect("At this point the guess should always be one character");

            match word_to_guess.find(guess) {
                Some(i) => {
                    println!("Found letter at {}", i);

                    let mut temp_state: Vec<char> = current_game_state.chars().collect();
                    temp_state[i] = guess;
                    current_game_state = temp_state.iter().collect();

                    let mut temp_word: Vec<char> = word_to_guess.chars().collect();
                    temp_word[i] = '_';
                    word_to_guess = temp_word.iter().collect();

                    found_chars_counter += 1;
                }
                None => (),
            }

            println!("Current game state: {}", current_game_state);

            if found_chars_counter == word_to_guess.len() {
                println!(
                    "You beat the game! The word we are looking for is: {}",
                    current_game_state
                );
                std::process::exit(0);
            }
        } else {
            println!("{} is not a letter!", guess);
        }
    }
}

// Src: https://github.com/davidbegin/clear-terminal
fn clear_screen() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
