//https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn main() {
    println!("Control flow tutorial");

    // ------------------------------
    // IF
    // usual expression :
    let n = 6;
    if n % 4 == 0 {
        println!("Number is at least divisible by 4");
    } else if n % 3 == 0 {
        println!("Number is at least divisible by 3");
    } else if n % 2 == 0 {
        // will not be printed because of the above expression
        println!("Number is at least divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
    // simple expressions
    if n % 4 == 0 {
        println!("Number is divisible by 4");
    }
    if n % 3 == 0 {
        // will be printed
        println!("Number is divisible by 3");
    }
    if n % 2 == 0 {
        println!("Number is divisible by 2");
    }

    // TERNARY OPERATOR
    let x = if n == 6 { "valid" } else { "invalid" };
    println!("Ternary operator result is: '{}'", x);

    // ------------------------------
    // LOOPS
    // loop
    // It is the infinite loop expression
    // it has to contain a break condition
    // simple loop :
    let mut counter = 0;
    println!("Starts loop");
    loop {
        println!("looped !");
        break;
    };
    println!("Ends loop");
    // loop with a result :
    let mut counter = 0;
    println!("Starts loop");
    let result = loop {
        println!("looped !");
        counter += 1;
        if counter >= 3 {
            // break can be used with or without an expression to return a value
            // since 'loop' is an expression, we can use it to affect a new value to 'result'
            break counter;
        }
    };
    println!("Ends loop");
    println!("Looped {} times", result);
    // loop labels
    // You can optionally specify a loop label on a loop that you can then use with break or
    // continue to specify that those keywords apply to the labeled loop instead of the innermost
    // loop.
    // Loop labels must begin with a single quote.
    // The following labeled loop count from 3 to 0, increments 1 to a counter every time and
    // finally ends by doing it 3 times.
    let mut counter = 0;
    println!("Starts labeled loop");
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 2;

        loop {
            println!("remaining = {remaining}");
            // break innermost loop if 'remaining == 0'
            if remaining == 0 {
                break;
            }
            // break 'counting up' loop if 'counter == 3'
            if counter == 3 {
                break 'counting_up;
            }
            // remove 1
            remaining -= 1;
        }
        // increase counter every time 'remaining' reaches 0
        counter += 1;
    }
    println!("End count = {counter}");

    // While
    // It is the conditional loop
    // you can also use 'break' statements into a while loop
    println!("Start counting until 0 using 'while' loop :");
    let mut count = 3;
    while count != 0 {
        println!("{count}");
        count -= 1;
    }
    println!("LIFTOFF!!!");

    // For
    // loop over elements, indexes or ranges
    // it is faster for index inbounds, because the compiler does not need to perform the
    // conditional check of whether the index is within the bounds on every iteration inside the
    // loop implementation.
    // Example :
    let array = [1, 2, 3];
    println!("Start counting to 3 using 'for' loop :");
    for element in array {
        println!("{}", element);
    }
    // other method for the countdown to 0 (alternative to 'while') :
    println!("Start countdown using 'rev' :");
    for element in (1..4).rev() {
        println!("{}", element);
    }
    println!("LIFTOFF!!")
}