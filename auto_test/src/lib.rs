#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(x: i32) -> i32 {
    x + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 must be equal to 4."))
        }
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 5
        };

        let smaller = Rectangle {
            width: 7,
            height: 3
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn time_consuming_test() {
        //Time consuming test that has been ignored.
    }
}

//By default, rust runs tests in parallel. Hence, tests that depend on each other would not work properly.
//One solution is to set a flag that tells rust to run the tests consecutively...like -- --test-threads=1

//Rust also disables or captures stdout upon successful tests. To tell it to print out all outputs, set the flag using...-- --show-output
//You could also specify the name of a single test to run by calling cargo test *test_name* such as cargo test larger_can_hold_smaller
//You could also filter out tests by specifying a string and all test names that match that string will be run.
//To tell Rust to ignore some tests, you could annotate the test with #[ignore] attribute, such as in the expensive test above.
//Rust would automatically ignore such tests.
//To run only ignored tests, you run the test with cargo test -- --ignored.

//To run only integration tests, you use the --test flag and specify the name of the integration file like ... cargo test --test integration_test
//Integration tests test how the various functions and APIs work when connected to together.
//Integration tests are put inside the tests directory. They aren't annotated with the #[cfg(test)] attribute.