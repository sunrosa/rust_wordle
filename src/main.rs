use colored::{self, Colorize};
use rand::seq::SliceRandom;

fn main() {
    wordle();
}

/// Correctness of a letter used in a position.
#[derive(PartialEq)]
enum Correctness {
    /// The correct letter is in the correct position.
    Correct,

    /// A correct letter is in an incorrect position.
    CorrectLetter,

    /// An incorrect (and unused) letter is used in an incorrect position.
    Incorrect,
}

/// Result of the game that was played.
enum GameResult {
    /// The player won.
    Success,

    /// The player lost.
    Failure,
}

/// Wordle configuration.
struct Configuration {
    /// Number of guess tries before the game is over.
    guess_tries: u32,

    /// Number of letters in the word to be guessed.
    guess_letters: u8,
}

fn wordle() {
    // Define program config
    let config = Configuration {
        guess_tries: 6,
        guess_letters: 5,
    };

    let possible_words: Vec<String>;

    {
        let all_words: &str = include_str!("wordle-nyt-answers-alphabetical.txt");
        possible_words = words_list(all_words, &config);
    }

    let mut result = GameResult::Failure;

    // Array of guessed letters in order from A to Z, and the number of instances of the letter in the target word
    let mut target_letter_count: std::collections::HashMap<char, u8> =
        std::collections::HashMap::new();

    // Possible words to be the target word to guess

    // The word the player is trying to guess
    let target_word = possible_words
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    // Number of guesses made by the player during the game
    let mut guesses = 0;

    print_tried_letters(&target_letter_count);
    print!(" ");

    // Game loop (break on max guesses)
    while guesses < config.guess_tries {
        guesses += 1;

        let mut correctness: std::collections::HashMap<u8, Correctness> =
            std::collections::HashMap::new();

        // Number of times a certain letter has been printed as contained in the target word
        let mut letter_count: std::collections::HashMap<char, u8> =
            std::collections::HashMap::new();

        // Prompt user input
        print!("({}/{})> ", guesses, config.guess_tries);
        flush();

        // Read user input
        let mut input: String = text_io::read!("{}\n");

        // Strip user input of additional whitespace, including newlines
        input = input.trim().to_lowercase().to_string();

        // Catch incorrect number of letters in guess, and refund the guess try
        if input.len() != config.guess_letters as usize {
            guesses -= 1;
            println!("The word is {} letters in length.", config.guess_letters);
            continue;
        }

        // Catch invalid words and refund guess try
        if !(&possible_words).into_iter().any(|w| w == &input) {
            guesses -= 1;
            println!("Please use a valid word.");
            continue;
        }

        // Operations on input
        for i in 0..input.chars().count() {
            let letter = input.as_bytes()[i] as char;

            // Count letters in input
            let count = target_word.chars().filter(|c| c == &letter).count() as i8;
            target_letter_count.entry(letter).or_insert(count as u8);
        }

        // Mark correctly placed letters
        for i in 0..input.chars().count() {
            let letter = input.as_bytes()[i] as char;

            if letter == target_word.as_bytes()[i] as char {
                // If letter is in the correct position
                *letter_count.entry(letter).or_insert(0) += 1;
                correctness.entry(i as u8).or_insert(Correctness::Correct);
            }
        }

        // Mark existing letters and letters not in target word
        for i in 0..input.chars().count() {
            let letter = input.as_bytes()[i] as char;

            // Mark letters that exist in target word
            if target_word.chars().any(|c| c == letter) {
                // If letter exists in target word
                if letter_count.entry(letter).or_insert(0)
                    < target_letter_count.entry(letter).or_insert(0)
                    && letter != target_word.as_bytes()[i] as char
                {
                    // Letter has not already been marked as existing (if there are more than one)
                    *letter_count.entry(letter).or_insert(0) += 1;
                    correctness
                        .entry(i as u8)
                        .or_insert(Correctness::CorrectLetter);
                } else {
                    // Letter has already been marked as existing (if there are more than one)
                    correctness.entry(i as u8).or_insert(Correctness::Incorrect);
                }
            }
            // Mark letters that are not in target word
            else {
                // If letter is not in target word
                correctness.entry(i as u8).or_insert(Correctness::Incorrect);
            }
        }

        for i in 0..input.chars().count() {
            match correctness.entry(i as u8).or_insert(Correctness::Incorrect) {
                Correctness::Correct => {
                    print!("{}", String::from(input.as_bytes()[i] as char).green())
                }
                Correctness::CorrectLetter => {
                    print!("{}", String::from(input.as_bytes()[i] as char).blue())
                }
                Correctness::Incorrect => {
                    print!("{}", String::from(input.as_bytes()[i] as char))
                }
            }
        }
        println!();

        // Mark success and end game loop if word is guessed
        if input == target_word {
            result = GameResult::Success;
            break;
        }

        // Prints tried letters (DOES NOT FLUSH, buffer is flushed on next input prompt call)
        print_tried_letters(&target_letter_count);

        // Insert space after letter list before input prompt
        print!(" ")
    }
    println!("");
    // Finalize the game
    match result {
        GameResult::Success => {}
        GameResult::Failure => {
            println!("The word was: {}", target_word)
        }
    }
}

fn flush() {
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

fn words_list(all_words: &str, config: &Configuration) -> Vec<String> {
    all_words
        .split('\n')
        .skip(2)
        .map(sanitize_word)
        .filter(|line| line.len() == config.guess_letters as usize)
        .collect()
}

/// Print list of tried letters from A to Z
fn print_tried_letters(target_letter_count: &std::collections::HashMap<char, u8>) {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    for letter in alphabet {
        if target_letter_count.contains_key(&letter) && target_letter_count[&letter] == 0 {
            print!("{}", String::from(letter).red());
        } else if target_letter_count.contains_key(&letter) && target_letter_count[&letter] > 0 {
            print!("{}", String::from(letter).blue())
        } else {
            print!("{}", String::from(letter));
        }
    }
}
