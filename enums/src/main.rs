// This enum definition is an example of using many different types of data, which is also
// the equivalent of defining multiple struct definitions, but they are defined under a single
// enum. This makes using the data much simpler.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can use any kind of data into an enum variant, including structs and other enums
enum IpAddr {
    V4(String),
    V6(String)
}

// Example: Using basic Enum as kind and Struct to Hold Data
//enum IpAddrKind {
//    V4,
//    V6,
//}

// Example: Defining a struct with enum kind
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

// Example: Defining function with enum
//fn route(ip_kind: IpAddrKind) {}

// Simple pattern matching with coins
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    New_Hampshire
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Pattern matching using Option variant
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Example: Creating Variable with Struct
    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};
    
    // Create v4/v6 IP Addresses
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Create coins and use pattern matching to determine type and value
    let coin = Coin::Penny;
    value_in_cents(coin);

    let quarter = Coin::Quarter(UsState::New_Hampshire);
    value_in_cents(quarter);    

    // Practice using pattern matching with option variant
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    
    let config_max = Some(3u8);
    // Example: Below pattern matching versus using 'if let'
    //match config_max {
    //    Some(max) => println!("The maximum is configured to be {}", max),
    //    _ => (),
    //}

    // Using 'if let' lets you pattern match against specific conditions without having
    // to write verbose pattern matching
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    
}