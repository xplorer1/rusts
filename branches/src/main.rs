fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3
        }
    };

    println!("The result is {}", result);

    looper();
    lift_off();
}

fn looper() {
    let arrr = [10, 20, 30, 40, 50];

    for elem in arrr.iter() {
        println!("element is {} ", elem);
    }
}

fn lift_off() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}


//in if-statements, the conditions aren't put into parenthesis.
//The conditions must evaluate to a boolean type. 
//Unlike javascript that attempts to convert non-boolean types such as integers to boolean.
