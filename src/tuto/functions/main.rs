// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

// Rust is an expression-based language.
// But function bodies are made up of a series of statements optionally ending in an expression.
// This is an important distinction to understand.
//   - Statements are instructions that perform some action and do not return a value.
//     Examples:
//       'let x = 1;' is a statement
//       'let x = (let y = 1); raises an error because 'let y = 1' is a statement and returns no value to 'x'.
//   - Expressions evaluate to a resultant value.
//     Expressions does not have a semicolon at the end.
//     If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
//     Examples:
//     '5 + 6' Expression that returns 11.
//     'let x = 11;' is a statement, but it uses the expression '11' in it ('11' returns '11').
//     Calling a function is an expression.
//     Calling a macro is an expression.
//     Scope blocks that returns a value are expressions, like '{11}'.
// The entire functions definitions are also statements.
// For example 'say_hello' from the first bracket to the last one is a statement of statements.
fn main() {

    // The following code block is an expression that returns 4.
    // {
    //     let x = 3;
    //     x + 1
    // }

    // The following code block is a statement that uses the expression returning 4
    let x = {
        let y = 3;
        // without ';' on purpose : this is an expression.
        // adding ';' turns it into a statement.
        y + 1
    };
    println!("Variable 'x' equal to '{}' thanks to the expression.", x);

    // Rust code uses snake case for names
    say_hello();

    // Return using an expression
    let five = get_five();
    // the following line does exactly the same without a function
    let f = 5;
    println!("The expression returned : '{}'", f);
    println!("The function returned : '{}'", five);

    // using parameters
    let x = 1;
    let y = 2;
    let result = my_addition(x, y);
    println!("'{} + {} = {}'", x, y, result);

    // The variable holding the result defines the mutability of the result, but not the function :
    let immutable_result = my_addition(1, 2);
    let mut mutable_result = my_addition(1, 2);
    mutable_result = 5;
    println!("The immutable result is: {immutable_result}");
    println!("The mutable result was changed to: {mutable_result}");
}

// Rust does not care where you define your functions.
// Only that they’re defined somewhere in a scope that can be seen by the caller.
fn say_hello() {
    println!("Hello world !")
}

// Always returns five using an expression
fn get_five() -> u8 {
    // returns implicitly '5' using an expression.
    5
    // using a statement will return an error:
    //5;
}

// Take parameters.
// Returns a value.
// The returned value is either mutable or immutable, the function doesn't define this behavior.
// It is the variable holding the result that defines the mutability of the result.
// The return value of the function is synonymous with the value of the final expression in the
// block of the body of a function.
// Most functions return the last expression implicitly
// You can return early from a function by using the return keyword and specifying a value
fn my_addition(x: i32, y: i32) -> i32 {
    x + y // implicit return using an expression
}