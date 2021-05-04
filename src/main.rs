use std::io;

fn main() {
    println!("Put in a word to guess!");

    let mut word_to_guess = String::new();

    io::stdin()
        .read_line(&mut word_to_guess)
        .expect("Failed to read input");

    word_to_guess = word_to_guess.trim().to_string();

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
            guess = guess.to_lowercase().to_string().chars().next().unwrap();
            word_to_guess = word_to_guess.to_lowercase();

            match word_to_guess.find(guess) {
                Some(i) => {
                    println!("Found letter at {}", i);

                    let mut temp_state: Vec<char> = current_game_state.chars().collect();
                    temp_state[i] = guess;
                    current_game_state = temp_state.iter().collect();

                    let mut temp_word: Vec<char> = word_to_guess.chars().collect();
                    temp_word[i] = '_';
                    word_to_guess = temp_word.iter().collect();

                    println!("Current game state: {}", current_game_state);

                    found_chars_counter += 1;
                }
                None => continue,
            }

            if found_chars_counter == word_to_guess.len() {
                println!(
                    "You beat the game! The word we look for is: {}",
                    current_game_state
                );
                std::process::exit(0);
            }
        } else {
            println!("{} is not a letter!", guess);
        }
    }
}
