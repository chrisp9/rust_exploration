extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

use std::io::Read;
use std::fs::File;

mod news_aggregator;
use news_aggregator::*;
use std::fmt::Display;

use std::thread;
use std::time::Duration;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

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

 fn largest<T : PartialOrd>(list: &[T]) -> &T {
     let mut largest = &list[0];

     for ref item in list.iter() {
         if *item > largest {
             largest = &item;
         }
     }

     largest
 }

struct Point<T> {
    x : T,
    y : T
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style:String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker")},
        Shoe { size: 3, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")}
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(in_my_size, 
        vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")}
        ]);
}





fn closure_fun() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T> where T : Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let g = self.value.entry(arg).or_insert((self.calculation)(arg));
        g.clone()
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn closure_capturing_context() {
    let x = 4;
    let equal_to_x = |z : u32| z == x;
    let y = 4;
    
}

fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let res: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
}

fn main() {

    closure_fun();

    strings();
    hashmaps();
    read_username_from_file();
    lifetimes();
}

fn lifetimes() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn news_aggregate() {
    let tweet = news_aggregator::Tweet {
        username: String::from("DonaldTrump"),
        content: String::from("Wall"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
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

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        }
        else {
            None
        }
    }
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
