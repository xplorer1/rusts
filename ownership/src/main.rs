fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello"); //s1 comes into scope.

    let (s2, len) = calculate_length(s1); //it is moved into s2

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) //because we to return the value of what was passed to us.
}

//data stored on the stack must have a fixed size.
//values without fixed size or whose size might change during runtime must be stored on the heap
//the heap is less organized. data is stored by allocating a space on the memory that is big enough for the data size and returning a pointer to it.
//rust never automatically copies deeply of heap data. Hence copy functions can be assumed to be inexpensive in terms of runtime perfomance.
//to guard against double free error, rust only keeps one reference to a heap data and invalidates any other.
//to make a copy of a heap data, use the clone method on the variable pointer. But this action can be expensive.
//the same rules apply when we pass values to functions.
//values that implement the Copy trait(values that don't need to be cloned before its data can be copied), can still be used after being passed into functions.
//but the opposite, variables that implement the Drop trait will have their values moved into functions.
//assigning a variable to another variable moves it.
//when a variable includes data on the heap goes out of scope, the value would be cleaned by drop unless the data has been moved/owned by another variable.

//references can solve this problem...the problem of using tuples to get back the values of what functions took ownership of.
//you pass in the address or reference of such value instead of the value. You do this by using the & character.
//This should be used at both the parameter declaration and when passing the argument.
//with it, you can refer to some value without taking ownership of it.

//just as variables are immutable by default, so are references. We are not allowed to modify something we have a reference to.
