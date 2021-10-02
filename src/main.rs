use std::cmp::Ordering;
use std::num::ParseIntError;
use rand::Rng;

const TOTAL_GUESSES_ALLOWED:usize = 10;

fn main() {
    play_guessing_game();
}

fn play_guessing_game() -> () {
    let mut guesses_made:[i32; TOTAL_GUESSES_ALLOWED] = [-1; TOTAL_GUESSES_ALLOWED];
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut total_guesses_made = 0;
    while total_guesses_made < TOTAL_GUESSES_ALLOWED {
        let guess:i32 = match get_guess_from_user() {
            Ok(num) => {
                guesses_made[total_guesses_made] = num;
                total_guesses_made += 1;
                num
            },
            Err(_) => {
                println!("Please enter a number!");
                continue
            },
        };
        check_if_user_won(guess, secret_number);
    }

    println!("Secret number: {}", secret_number);
    display_user_guesses(guesses_made);
}

fn get_guess_from_user() -> Result<i32, ParseIntError> {
    println!("Please enter a guess:");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    guess.trim().parse()
}

fn check_if_user_won(guess: i32, goal: i32) -> bool {
    println!("\nYou guessed: {}", guess);

    match guess.cmp(&goal) {
        Ordering::Less => {
            println!("You should guess higher!\n");
            false
        },
        Ordering::Greater => {
            println!("You should guess lower!\n");
            false
        },
        Ordering::Equal => {
            println!("Great Work! You guessed correctly!");
            true
        }
    }
}

fn display_user_guesses(guesses_made: [i32; TOTAL_GUESSES_ALLOWED]) {
    println!("{} Guesses made in total\n", guesses_made.len());
    let mut guess_number = 1;
    for guess in guesses_made.iter() {
        if guess == -1 {
            break;
        }
        println!("Guess #{}: {}", guess_number, guess);
        guess_number += 1;
    }
}