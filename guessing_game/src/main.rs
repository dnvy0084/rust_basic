use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    let mut count = 0;

    let mut i = 0;

    loop {
        println!("({})", i);

        i = i + 1;

        if i > 10 {
            break;
        }
    }

    // println!("secret number is {}", secret);

    loop {
        println!("plz, input your guess. ({})", count);

        count = count + 1;

        let mut read = String::new();
        let _result: std::io::Result<usize> = io::stdin().read_line(&mut read);

        _result.expect("Failed to read line");

        let guess: u32 = match read.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
