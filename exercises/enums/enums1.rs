// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

use std::string;

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x : i32, y : i32 },
    ChangeColor(i32,i32,i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello,cargo!")));
    println!("{:?}", Message::Move { x: 3, y: 4 });
    println!("{:?}", Message::ChangeColor(255,165,0));
}
