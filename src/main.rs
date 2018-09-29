extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

use std::io::Read;
use std::fs::File;

struct User {
    username: String,
    email : String,
    sign_in_count: u64,
    active: bool
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    strings();
    hashmaps();
    read_username_from_file();
}

fn hashmaps() {
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let key = String::from("Favorite Color");

    let item = map.get(&key).unwrap();
}

fn strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2;


}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file_contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn vector() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];

    let v_index = 2;

    match v.get(v_index) {
        Some(_) => {println!("Reachable element at index: {}", v_index)},
        None => {println!("Unreachable element at index: {}", v_index)}
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    };

}

fn if_let() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }
} 

fn value_in_cents(coin : Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn option() {
    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number : Option<i32> = None;
}

fn messages() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn rectangles() {
    let rect1 = Rectangle {width: 30, height: 50};

    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn coins() {

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
