
struct Point<T> {
    x: T,
    y: T
}

struct Point2<T, U> {
    x: T,
    y: U
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

impl<T> Point<T> {
    fn return_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn calculate_distance(&self) -> f32 {
        &self.x * &self.y
    }
}

pub trait Summarize {
    fn summarize(&self) -> String;
}

pub trait Summarize2 {
    fn summarize2(&self) -> String {
        format!("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String
}

pub struct Tweets {
    pub content: String,
    pub author: String
}

impl Summarize for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} at ({})", &self.headline, &self.author, &self.location)
    }
}

impl Summarize for Tweets {
    fn summarize(&self) -> String {
        format!("{}, by: {}", &self.content, &self.author)
    }
}

impl Summarize2 for NewsArticle {}

pub fn notify(item: &impl Summarize) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify_with_multiple_traits_different_types(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify_with_multple_traits_same_type<T: Summary>(item1: &T, item2: &T) {}

fn main() {
    println!("Hello, world!");

    //Generics is used to handle duplication of concepts. Most especially when have the same logic for different data types.
    //To use generics, place it in the signature of the function, where the data type is usually specified. Consider the code below. Both do exactly the same thing.
    //Only difference is in their data types.

    //let number_list = vec![3,4,5,8,9];

    //let largest_number = largest(&number_list);

    //print!("The largest number is: {}", largest_number);

    //Generics can also be applied on structs. The definition is similar to that of functions.
    //Note that since we have used on generic type on Point, all its members must be of that same type.
    //So this won't work... let plane = Point {x: 5.4, y: 6}. They must all have the same type.
    //If you want them to have different types, you have to specify more than one generic type.
    //So you define it as Point2.
    //Bear in mind that for Point2 struct, the members can all have one type.
    //You can also define enums for generic types as well.
    //You could define methods on the generic structs as well such as in the return_x function above.
    //Also note you could define method on structs that are only applied on specific types. such as the calculate_distance method above.
    //It implies that no other type would have access to the calculate_distance method besides f32.


    //Traits define shared behaviour among different types. A type's behaviour consists of methods we can call on that type.
    //Different types share the same behaviour if we can call the same methods on those types.
    //Traits are similar to interfaces in languages such as Java.

    /**
        Hence, any method or type that implements a a trait must implement the methods described in the trait.
        You describe a trait using the trait keyword, like the Summarize trait above.
        Any struct or method that implements the Summarize trait must provide logic for the summarize method in the trait.

        We can use the NewsArticle and Tweets structs above to illustrate the use of traits.

        After implementing traits, we can use the implementations on instances of the types they were defined on.
        such as in...
    **/

    //We can only implement a trait for a type only if the type or the trait is local to our crate.
    //We can also have a default implementation for a trait and then choose to either use the default implementation or override it.
    //In such cases where there is a default implementation, we can still add the trait on the type by using an empty implementation block like so...
    //Summarize2 above.

    //Traits can call other traits the same way a method might call other methods.

    //Traits as parameters. ////////////
    //You could pass in traits as parameters to functions...
    //Such as in notify function above...


    //Traits bound syntax. /////////
    //You can use trait-bound syntax to specify more than one trait, especially when you want them to have only one type.

    let plane = Point {x: 5, y: 3};
    let plane2 = Point2 {x: 5, y: 8.4};

    let news = NewsArticle {
        author: String::from("Ezeja Maximus"),
        content: String::from("This is news from maximums"),
        location: String::from("Abia State"),
        headline: String::from("The re-imagining of the new 21st century office work.")
    };

    println!("News article is: {}", news.summarize());

    println!("News article with default implementation: {}", news.summarize2());

    println!("Notify all participants: {:?}", notify(&news));
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//They both could be combined to form one generic function like below.

// fn largest<T> (list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }