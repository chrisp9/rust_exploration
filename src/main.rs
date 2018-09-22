extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    slices();


}

fn slices() {
    let s = String::from("hello world");

    let hello = &s[0..=4];
    let world = &s[6..=10];

    println!("{}", hello);
    println!("{}", world);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}


fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");     
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
               println!("Winner!");
               break;
            }
        }
    }
}
