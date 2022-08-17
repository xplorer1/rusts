use std::error::Error;
use std::fs;
use std::env;

pub fn run(config_params: ConfigParams) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config_params.filename).unwrap();

    let results = if config_params.case_sensitive {
        search(&config_params.query, &contents)
    } else {
        search_case_insensitive(&config_params.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct ConfigParams {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl ConfigParams {
    pub fn new(arguments: &[String]) -> Result<ConfigParams, &str> {
        if arguments.len() < 3 {
            return Err("Invalid number of arguments.");
        }

        let query = arguments[1].clone();
        let filename = arguments[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(ConfigParams {query, filename, case_sensitive})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
            //do something with line here.
        }
    }

    results
}

//Lifetime annotation is used to tell the generic lifetime parameters of multiple references relate to each other.
//To specify a lifetime, use an apostrophe after the borrow parameter, like &'a i32

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }