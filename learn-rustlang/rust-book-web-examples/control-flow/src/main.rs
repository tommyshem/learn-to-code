mod if_let;
mod if_let_else;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_to_string(some_u8_value: u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // match anything else
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("coin = {:?}", value_in_cents(coin));
    let some_u8_value = 3u8;
    value_to_string(some_u8_value);
}
