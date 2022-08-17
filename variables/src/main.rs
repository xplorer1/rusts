fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is now: {}", x);

    let age: u32 = 34;

    println!("Age is {} ", age);

    let tuple: (u64, f64, u64) = (200, 5.0, 2);

    println!("first guy: {}", tuple.0);
    println!("second guy: {}", tuple.1);
    println!("third guy: {}", tuple.2);

    let test = 9;
    let mut tester = 9;

    let test = 10;
    tester = 10;

    println!("first tester: {}", test);
    println!("second tester: {}", tester);
}

//signed variables mean they can have negative 
//and positive values. 
//Unsigned means they will always be positive values.

//Tuples is a general way of grouping items with a variety of types into one 
//compound type. They have fixed lengths. Can't be increased or decreased.

//arrays must have a fixed length. 
//they must also contain elements of the same type.
//vectors are similar to arrays but can have variable length.