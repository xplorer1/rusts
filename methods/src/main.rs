#[derive(Debug)]
struct Rectangle {
    height: f64,
    width: f64
}
impl Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle {
        height: 10.0,
        width: 5.0
    };

    println!("The area of react1 is {} ", rect1.area());

    let rect2 = Rectangle {
        width: 10.0,
        height: 40.0,
    };
    let rect3 = Rectangle {
        width: 60.0,
        height: 45.0,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
