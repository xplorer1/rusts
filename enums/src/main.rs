enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

//OR...

enum IpAddr2 {
    V4(String),
    V6(String)
}

struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home_addr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    //OR...

    let home = IpAddr2::V4(String::from("128.1.1.0.1"));

    let some_number = Some(50);
    //let absent_value: Option<i32> = None;
    #[derive(Debug)]
    let some_other_number: Option<i32> = Some(50);

    println!("some_number {:#?}", some_other_number);
}

//enums allow us to define a type by specifying its possible variants.
//enum values can only be one of its variants.
//the option enum lets us define a sort of null value.
//it encodes the scenario in which a value could be something or could be nothing.

enum Option<T> {
    Some(T),
    None,
}