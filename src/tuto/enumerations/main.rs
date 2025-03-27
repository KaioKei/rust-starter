//https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
//https://doc.rust-lang.org/book/ch06-02-match.html
// ENUM
enum IpAddrKind {
    // fields of an enum are called 'Variants'
    V4,
    V6,
}

// enums can also hold different types and definitions
// Defining an enum with variants like below is similar to defining different kinds of struct.
// The difference is the enum holds all the variants are grouped together under the 'Message' type.
enum Message {
    // simple variant
    Quit,
    // typed variant
    Write(String),
    // structure variant
    Move { x: i32, y: i32 },
    // tuple variant
    ChangeColor(i32, i32, i32),
}

// enums can be implemented, exactly like structures
impl Message {
    fn send(&self) {
        // write function body
    }
}
// If we used the different structs instead of one enum with different variants, each of which has
// its own type, we could not as easily define a function to take any of these kinds of messages :
//struct QuitMessage; // unit struct
//struct MoveMessage {x: i32, y: i32,} // scalar struct
//struct WriteMessage(String); // tuple struct
//struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // ENUM
    // It is important to note that the 2 following variables :
    // - have the same type IpAddrKind (enum)
    // - have different variants (v4, v6)
    let variant_v4: IpAddrKind = IpAddrKind::V4;
    let variant_v6: IpAddrKind = IpAddrKind::V6;

    // In this example, we’ve created a variable m that has the value
    // Message::Write(String::from("hello")), and that is what self will be in the body of the call
    // method when m.send() runs.
    let m = Message::Write(String::from("hello"));
    m.send();

    // OPTION
    // Rust does not have the 'null' feature.
    // In languages with 'null', variables can always be in one of two states: null or not-null.
    // The problem is that if you try to use a null value as a not-null value, you’ll get an error.
    // Rust does not have nulls, but it does have an enum type 'Option<T>' that encodes a value
    // which could be something or nothing.
    // Internally, the Enum type 'Option<T>' is expressed as follows, where T can be of any type :
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_number: Option<u8> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;
    println!("Uint as Option<u8> : {:?}", some_number);
    println!("Char as Option<char> : {:?}", some_char);
    println!("None Int as Option<i32> : {:?}", absent_number);
    // For example, with the type 'Option<T>' :
    //   - if you request the first item in a non-empty list, you would get 'Some(T)' value.
    //   - if you request the first item in an empty list, you would get a 'None', but not a null.
    // Thus, the Option functionality can prevent bugs that are extremely common
    let a: [u8; 3] = [1, 2, 3]; // Rust force you to set each item to a no-null value
    println!("First member of array of u8 : {}", a[0]);
    println!("Second member of array of u8: {}", a[1]);
    let a_option: [Option<u8>; 3] = [None, Some(2), Some(3)]; // Rust allows you to use 'Option' enum type to set a value to None, but it is a no-null value
    println!("First member of array of Option<u8> : {:?}", a_option[0]);
    println!("Second member of array of Option<u8>: {:?}", a_option[1]);

    // The compiler won’t let us use an Option<T> value as if it were definitely a valid value.
    // The following code would raise an error :
    //let x = some_number + a_option[1]; // You cannot use Option<T> directly as a scalar type
    // With type i8 in Rust, the compiler will ensure that we always have a valid value.
    // With type Option<i8>, the compiler will make sure we handled the case where the value is
    // not None before using the value.
    // In other words, you have to convert an Option<T> to a T before you can perform T operations,
    // eliminating the risk of incorrectly assuming a not-null value.

    // MATCH CONTROL FLOW - ENUM
    // NEXT : the match control flow construct

}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}