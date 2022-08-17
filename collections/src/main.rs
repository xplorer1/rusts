use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 3, 4];

    //let mut v1 = Vec::new();

    //v1.push(5);
    //v1.push(6);

    //let second_again = v1.get(2);
    let second_again = &v1[2];

    println!("second again is: {}", second_again);

    let mut s4 = String::from("hello ");
    s4.push_str("world");

    println!("Full string is: {}", s4);

    let name = greet_person(String::from("Chijioke Ugwuanyi"));

    println!("Name is {}", name);

    let s5 = String::from("Dice.");

    println!("First character is {}", &s5[0..1]);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

pub fn greet_person(name: String) -> String {
    name
}

//Collections can contain multiple values, like arrays and tuples.
//But unlike arrays and tuples, they are stored on the heap. This means that the size of data does not need to be known at compile time.

//There are 3 types of collections most often used in programming.

//Vector allows us to store a variable number of values next to each other.
//String is a collection of characters.
//Hashmap allows us to associate a value with a particular key.

//Vectors are similar to javascript arrays.
//Vectors allow us to store more than one value in a single data structure that puts all of the values close to each other in memory.

//You can create a vector by

//let v: Vec<i32> = Vec::new();
//v = vec![1, 3, 4];

//let mut v1 = Vec::new();

//v1.push(5);
//v1.push(6);

//Rust can infer the type of a vector through type of the members of the vector.
//vectors can only store values of the same type.

//To push items into a vector, you can use the .push method. This is only possible if the vector/variable is mutable.

//A vector is dropped when it goes out of scope.

//To read a value of a vector, there are two ways.
//1. You use the reference operator & and [] brackets.
//So in v1 above, to get the second element, you'd do something like 
//let second = &v1[1];

//2. You use the get method on the vector.
//let second_again = v1.get(2);

//The first method is suitable for when you want your program to crash if it encounters a scenario such as when one tries to referenece a non-existent index.
//If you want to handle said situation, then you should use the get method which would return a None value.

//You can't have mutable and immutable references in the same scope.



//*
 //* STRINGS
 //Both String and string slices are UTF-8 encoded.

 //to create a new mutable string, you start with

//let mut s = String::new();

//let data = "Initial content";
//s = data.to_string();

//OR...

//let s2 = "initial content".to_string();

//OR...
//let mut s3 = String::from("Initial content.");
//A string can grow in size if you add more data to it. Hence, it is stored on the heap.

//s3.push_str(" that has been added to.");

//the method push_str takes a string slice cos we don't want to take ownership of the of the parameter after calling the method. 
//So it takes a reference to the parameter.
//To push a character onto a string, you use the push method. This doesn't allow anything beyond a character.


//*HASHMAPS.
//*
//*

//All the keys of a hashmap must be of same type. All the values as well must be of the same type. That is to say that HashMaps are homogenous.
//For types that implement the copy trait such as i32, the values are merely copied into HashMaps. But owned types such as Strings, the values are moved and owned by the HashMap.
//Hence, they will no longer be accessible after the values have been moved.

//To get a value from a HashMap, you supply the key to the get method of the HashMap.
//let hash_map = HashMap::new();

//hash_map.insert(String::from("hundred"), 100);
//hash_map.insert(String::from("fifty"), 50);

//let first_value = String::from("hundred");

//hash_map.get(&first_value); ...this will return an Option<&V> which you have to handle.

//There are few ways to update values in a hash map.
//You can overwrite an existing value by...
//Simply inserting a value with an existing key. The old value will be overwritten.

//To insert a value to a key only if the key has no value, we use the entry method on the hash_map.
//This method takes the key we want to check and calls the or_insert method with the value to insert if no value was found.

//hash_map.entry(String::from("hundred")).or_insert(50);
