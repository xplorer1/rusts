fn main() {
    println!("Hello, world!");
}

//Cargo has a number of featuers used in managing code organization.
//Packages allow building and testing and sharing of related crates.
//Crates is a tree of modules that produce a library or an executable.
//Modules allow us to control the organization, scope and privacy of paths.
//Paths is a way of naming items such as structs, functions and modules.

//A crate is a binary or library. The root of a crate is a source file that rust compiler starts from when compiling your code.
//It also makes up the root module of the crate.

//A package is one or more crates that provide a set of functionality. A package contains a cargo.toml file that describes how to build those crates.
//A package must contain 0 or 1 library crate. But no more. Only one.
//It can contain as many binary crates as possible, but at least one crate. (Binary or library).

//By convention, when we create a package with _cargo new package-name_, main.rs is chosen as the crate root.
//A package can have multiple binary crates by placing files in the src/bin directory; each file will be a separate binary crate.

//Modules allow us to organize code within a crate into groups for readerbility and re-use.
//Also allow us to control the privacy of items.

mod front_of_house;

use crate::front_of_house::hosting; //this is same as using a relative path use self::front_of_house::hosting;
//to re-export this module, we could do ...
pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    //absolute path method
    hosting::add_to_waitinglist();

    //relative path method
    hosting::add_to_waitinglist();
}

//In the above front_of_house module, hosting and serving modules are siblings, while front_of_house is their parent.
//All modules inherit from the implicit crates module, just like all objects inherit from an implict Object parent class in OOP languages.
//Path refers to location of an item in a module tree --- Absolute and Relative paths.
//An absolute path starts from the crate root by using a crate name or a literal -- crate.
//A relative path starts from the current module and uses self or super or an identifier in the current module.
//Both make use of ::
//We can also use the super keyword to come out of the current module and refer to anothe item in another module

//For structs, using pub before a struct's definition makes it public, but not its contents.
//But this still makes its members private. You'd need to mark each of the structs members as public to have access to it.

//If we make an enum publi, then all of its variants are then public.

//We can use the _use_ keyword to bring a module into scope so we can use it as if it's a local module.

//It is good practice to have parent modules refer to their functions when the functions are being called. This shows where the functions are coming from.
//On the other hand, if we are bringing in structs or enums into scope using the use keyword, it follows convention to specify the full path. An exception occurs when you want to bring in two artifacts with same name. In that, you follow the function convention.
//We could use the _as_ keyword to solve the problem of importing two modules with same name.

//we could combine pub and use to re-export a module.

//to bring in different modules with same parent, we can use nested import method.
use std::cmp::Ordering;
use std::io;

//..could both be shortened to...
use::{cmp::Ordering, io};

//to bring all public items in a path into scope,you can use the _*_ operator.
use::std::*

//.. to import all public items under std.