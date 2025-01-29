use rand::Rng;
use std::io;
use std::io::Write;



fn main() {
    let mut user_score: i64 = 0; 
    let mut playing: bool = true;


    game(&mut user_score);
    while playing {
        let mut continue_prompt = String::new(); // inside the loop so that it resets every time
        // should expect "Y" to keep playing, else it will kill the loop
        println!("Play again? (Y/N)");
        io::stdin().read_line(&mut continue_prompt).expect("Failed to read line");
        if continue_prompt.trim().to_uppercase() != "Y" {
            println!("Thank you for playing");
            playing = false;
        } else {
            game(&mut user_score);
        }
    }
}

fn game(user_score:&mut i64) {

    let secret_number = rand::thread_rng().gen_range(1..101); // 1 to 100

    let mut user_guess = String::new(); // create room for the user input
    print!("Enter your guess: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Ensure the output is printed before user input (rust thing, otherwise u might just get an input immediately)
    io::stdin().read_line(&mut user_guess).expect("Failed to read line"); // read the input and expect it to be a string

    let user_guess: i64 = user_guess.trim().parse().expect("Please type a number!"); // convert the string to a number, if that fails, they did not type a number
    let difference: i64 = (secret_number - user_guess).abs(); // like pythons abs() just at the end

    println!("Secret number is {secret_number}. You guessed {user_guess}. Difference is {difference}.");

    if difference == 0 {
        println!("JACKPOT!!! You score 100 points");
        *user_score += 100;
    } else if difference <= 20 {
        println!("You score 20 points");
        *user_score += 20;
    } else {
        println!("You lose 30 points");
        *user_score -= 30;
    }

    println!("Your total score is: {user_score}");
}