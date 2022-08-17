enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");

    let return_value = value_in_cents(Coin::Quarter);

    println!("The return value is: {}", return_value);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Found a three!"),
        _ => ()
    }

    //Which is same thing as ...
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

//match expressions return any type of value.
//to match expressions that match only one pattern while ignoring the rest,
//use if and let combinations.
