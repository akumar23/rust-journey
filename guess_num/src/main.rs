use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1-100");
    
    //generate random number 
    let num = rand::thread_rng().gen_range(0..100);
    
    let mut count = 0;

    loop {

        println!("Please input guess");
        let mut guess = String::new();
        count += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_num: i32 = guess.trim().parse()
                                    .expect("Input isn't an number");

        match guess_num.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you {count} guesses!");
                break;
            }
        }
    }
}
