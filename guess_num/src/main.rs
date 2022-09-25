use std::io;
use rand::Rng;


fn main() {
    println!("Guess a number between 1-100");

    println!("Please input guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess_num: i32 = guess.trim().parse().expect("Input isn't an integer");
    
    //generate random number 
    let num = rand::thread_rng().gen_range(0..100);

    if num == guess_num {
        println!("You guessed right!! The number was: {guess}");
    } else {
        println!("Sorry you guessed wrong. The number was: {}", num);
    }

}
