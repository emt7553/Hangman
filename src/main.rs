use std::io::{self, Write};

fn main() {
    let word = "goated";
    let mut guessed_letters = Vec::new();
    let mut incorrect_guesses = 0;


    loop { 
        display(word, &guessed_letters);
        print!("Guess a letter: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim().to_lowercase().chars().next(); 
        
        let chr = match guess {
            Some(letter) => letter,
            None => {
                println!("Enter a valid letter:"); 
                break;
            },
        };
        if !guessed_letters.contains(&chr){
            guessed_letters.push(chr);
        }
        else {
            println!("You have already guessed that letter!");
            continue;
        }
        

        if word.contains(chr) {
            
        }
        else {
            incorrect_guesses += 1;
            let guesses_left = 6-incorrect_guesses;
            println!("No {chr} found! {guesses_left} attempts left!");
            continue;
        }

        if word.chars().all(|c| guessed_letters.contains(&c) ) {
            println!("You win! Word was: {word}");
            break;
        } 
        else if incorrect_guesses == 6 {
            println!("you lose im not gonna tell u the word");
            break;
        }
    }

}

fn display(word: &str, guessed_letters: &Vec<char>) {
    for letter in word.chars() {
        if guessed_letters.contains(&letter) {
            print!("{letter}");
        }
        else {
            print!("_")
        }
    }
    println!("");
    println!("You have guessed: {:?} ", &guessed_letters[..]);
    
}
