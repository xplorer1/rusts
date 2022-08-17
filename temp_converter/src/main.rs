use std::io;

fn main() {
    println!("Temperature converter!");

    let mut temp_to = String::new();
    let mut temp = String::new();

    println!("Enter 'F' or 'C' to convert to fahrenheit or celsius respectively.");

    io::stdin()
        .read_line(&mut temp_to)
        .expect("You have entered a wrong input!");
        
    println!("Enter the value you want to convert.");
    
    io::stdin()
        .read_line(&mut temp)
        .expect("You have entered a wrong input!");

    let temp: f64 = temp.trim().parse().expect("What a dufus! Give me a fucking number!");

    let temp_to: char = temp_to.trim().parse().expect("What a dufus! Give me a fucking char!");

    if temp_to == 'F' {
        let result = to_fahrenheit(temp);

        println!("The value of {} celsius in faherenheit is {} ", temp, result);
    } else {
        let result = to_celsius(temp);

        println!("The value of {} faherenheit in celsius is {} ", temp, result);
    }
}

//to Fahrenheit
fn to_fahrenheit(temp: f64) -> f64 {
    println!("gotcha {}", temp);

    (1.8 * temp) + 32.0
}

//to celsius
fn to_celsius(temp: f64) -> f64 {
    ((temp - 32.0) * 5.0)/9.0
}