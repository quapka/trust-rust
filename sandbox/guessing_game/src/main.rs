use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

#[test]
fn test_generate_secret_number_range() {
    for _ in [0..1000].iter() {
        let number: u32 = generate_secret_number();
        assert!(1 <= number);
        assert!(number <= 100);
    }
}

fn main() {
    println!("Guess the secret number!");

    println!("Please input your guess: ");

    let secret_number = generate_secret_number();

    loop {
        print!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Warning: the input must be a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
