use std::fs::File;

fn main() {
    println!("Hello, world!");

    //In Rust exceptions are grouped into two types: Recoverable and Unrecoverable errors.

    //Recoverable errors are usually execptions you want to handle and notify a user or an application.
    //Unrecoverable errors are those that cause the program to crash or panic.
    //In Rust, recoverable errors are handled using Result<T, E>, while unrecoverable errors can be triggered using the panic! macro...such as...

    //let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    //You could replace the above match pattern with the Result struct's unwrap and expect methods.

    //let file = File::open("hello.txt").expect("Provide a file you dumbass!");

    //let file = File::open("hello.txt").unwrap();
}
