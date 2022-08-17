use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let name = "Eze";
    println!("This is the guess the number game!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("You have entered a wrong input!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("What a dufus! Give me a fucking number!");
                continue;
            }
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Secret number is {}: Your guess is: {}!", secret_number, guess);
                break;
            }
        }
    }
}