struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

//struct is like objects in javascript.
//structs are like tuples...pieces of a struct data can be of different types.
//but unlike tuples, you'll name each of the types so it's clear what the values mean.


fn main() {
    //println!("Hello, world!");

    // let user1 = User {
    //     username: String::from("someusername123"),
    //     email: String::from("some@example.com"),
    //     active: true,
    //     sign_in_count: 2
    // };

    //let user = build_user(String::from("bigboy@smallboy.com"), String::from("bigboysmallboy123"));

    //let length = 15.0; //this is by using the newbie method.
    //let width = 8.5; //this as well.

    let react1 = Rectangle {
        width: 10.0,
        height: 5.0
    };

    println!("The parameters ares: {:#?}", react1);

    println!("Area of the rectangle is: {}", rectangle_area(&react1));
}

fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username,
        active: true,
        sign_in_count: 5
    }
}

fn rectangle_area(reactangle: &Rectangle) -> f64 {
    reactangle.height * reactangle.width
}