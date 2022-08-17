fn main() {
    println!("Hello, world!");

    println!("Another val: {} ", another_function());
}

fn another_function() -> u64 {
    println!("This is anotehr function!");
    let value = returner(34);

    value
}

//statements perform some action but do not return any value.
//expressions return some value/evaluate to some value.
//expressions do not include ending semi-colons as that would turn it into a statment.

//you declare a function that returns a value like:

fn returner(x: u64) -> u64 {
    x
} 