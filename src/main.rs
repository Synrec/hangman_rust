use std::io;
use rand::Rng;

fn main() {
    println!("Welcome! Do you want to play a game? (y/n)");
    let mut bad_input: u64 = 0;
    loop {
        let mut play_game = String::new();
        io::stdin()
            .read_line(&mut play_game)
            .expect("Failed to read input!");
        let play_game = play_game.trim();
        if play_game == "y"{
            println!("*chuckles* And so the game begins!");
            hangman_game();
            break
        }else if play_game == "n"{
            println!("What a shame...");
            break;
        }else{
            bad_input += 1;
            let mut evil = bad_input;
            while evil > 0{
                println!("...");
                evil -= 1;
            }
            println!("...Yes = y and No = n. {play_game}: is not valid");
        }
    }
}

fn hangman_game (){
    let hangman_words = [
        "apple",
        "beetle",
        "cat",
        "dog",
        "eggplant",
        "fish",
        "gobble"
    ];
    let word_index: usize = rand::thread_rng().gen_range(0..=6);
    let hangman_word = String::from(hangman_words[word_index]);
    println!("Your word has been chosen for you!~");
    run_hangman(hangman_word);
}

fn run_hangman (word: String){
    let mut lives = 5;
    let mut guessed_word = String::from("");
    loop{
        if lives <= 0 {
            println!("Correct Word: {word}");
            println!("You can't play anymore ~T_-~");
            break;
        }
        let mut word_length: usize = word.len();
        let mut valid_chars = String::new();
        while word_length > 0{
            let letter: char = word.chars().nth(word_length - 1).expect("No letter character");
            if guessed_word.contains(letter) {
                valid_chars.push(letter);
            }
            word_length -= 1;
        }
        let mut valid_counter : usize = 0;
        let mut current_guessed_word = String::new();
        for c in word.chars(){
            if valid_chars.contains(c){
                current_guessed_word.push(c);
                valid_counter += 1;
            }else{
                current_guessed_word.push('_');
            }
        }
        if valid_counter >= word.len(){
            println!("Correct Word: {word}");
            println!("Hey, you won. Give yourself a pat on the back eh?");
            break;
        }
        println!("You have {lives} remaining. Good luck!~");
        println!("Current: {current_guessed_word}");
        println!("Current Guesses: {guessed_word}");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input. How sad ~T_T~");
        let guess = guess.trim();
        println!("Your guess is {guess}.");
        if guess.len() > 1{
            println!("Nuh uh uh! You can't guess more than one letter!");
            println!("That's going to cost you!~");
            lives -= 1;
            continue;
        }else{
            let mut cont : bool = false;
            for c in guess.chars() {
                if guessed_word.contains(c){
                    println!("Hey! You guessed that letter already!");
                    println!("You won't get punished but be careful!");
                    cont = true;
                }
            }
            if cont{
                continue;
            }
            for c in guess.chars() {
                guessed_word.push(c);
            }
            lives -= 1;
        }
    }
}