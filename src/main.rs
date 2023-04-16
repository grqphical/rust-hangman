use rand::seq::SliceRandom;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Reads a file called words.txt and loads all the useable words and returns them as a Vector of strings
fn read_words() -> Vec<String>{
    let file = File::open("words.txt").unwrap();
    let reader = BufReader::new(file);

    let mut words = Vec::new();

    for line in reader.lines() {
        let word = line.unwrap();
        words.push(word);
    }

    return words;
}

// Function used to print a vector nicely to the console
fn print_vector(vector: &Vec<char>, title: &str)
{
    print!("{title}");
    for item in vector{
        print!("{item} ")
    }
    print!("\n");
}

fn main() {
    // Initalize our variables
    let list_of_words = read_words();

    let mut guesses_left = 10;

    // Choose a random word
    let word_to_guess = list_of_words.choose(&mut rand::thread_rng()).expect("Error when generating random value");
    let mut current_guess: Vec<char> = vec!['_'; word_to_guess.len()];
    let mut incorrect_letters_guessed: Vec<char> = Vec::new();

    // Print the intro screen
    println!("Welcome to hangman. Try to guess the word");
    loop {
        // Convert the string to a vector for comparison
        let word_as_vec: Vec<char> = word_to_guess.chars().collect();

        // Check if the player has won and if so end the game
        if current_guess == word_as_vec{
            println!("You win the word was: {word_to_guess}");
            break;
        }

        // Handle the game over condition
        if guesses_left == 0{
            println!("Game Over the word was: {word_to_guess}");
            break;
        }

        // Print the player's current stats such as guesses left and what letters they have
        print_vector(&current_guess, "Current Guess: ");
        print_vector(&incorrect_letters_guessed, "Letters not in the word: ");
        println!("Guesses left: {guesses_left}");

        // Prompts player for letter
        let mut guess = String::new();
        println!("Guess a letter: ");
        io::stdin().read_line(&mut guess).expect("failed to readline");

        // If letter isn't in word, remove a guess, add the letter to the list of letters that aren't in the word
        if !word_to_guess.contains(guess.chars().next().unwrap())
        {
            // Only add it if it doesn't exist already
            if !incorrect_letters_guessed.contains(&guess.chars().next().unwrap()){
                incorrect_letters_guessed.push(guess.chars().next().unwrap());
            }
            guesses_left -= 1;
            continue;
        }

        // Loop through the word to get the position of the character and add it to the current_guess vector
        for (i, c) in word_to_guess.chars().enumerate()
        {

            if guess.chars().next().unwrap() == c
            {
                current_guess[i] = c;
                continue;
            }
        }
        
        
    }

}
