// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
// https://doc.rust-lang.org/book/ch03-02-data-types.html
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html (for strings)
fn main() {

    // ------------------------------
    // VARIABLES
    // Rust code uses snake case as the conventional style for names
    // they are immutable by default
    // they cannot be changed by the code
    let immutable_variable = 1;
    println!("The value of 'immutable' is: {immutable_variable}");
    //immutable = 2; // Cannot work because 'x' is immutable

    // MUTABLE VARIABLES
    // defined by 'mut'
    // they can be changed by the code
    // their type cannot be changed
    let mut mutable_variable = 1;
    println!("The first value of 'mutable' is: {mutable_variable}");
    mutable_variable = 2;
    println!("The second value of 'mutable' is: {mutable_variable}");
    //mutable = "error"; // won't work because we are not allowed to change the type

    // CONSTANTS
    // they are not just immutable by default, they are always immutable
    // the type of the value MUST BE annotated
    // they cannot be declared with 'mut' (they are always immutable)
    // they can be declared in any scope, useful for global scopes
    // constants may be set only to a constant expression, not the result of a value that could
    //      only be computed at runtime.
    // any expression of a constant is evaluated during compilation,
    //      so the expression should always be explicit.
    // CHECK WHAT ARE EXPRESSIONS IN THE 'FUNCTIONS' TUTORIAL.
    const BASIC_CONST: u32 = 1;
    println!("Constant set to '{BASIC_CONST}'");
    // the following constant is evaluated during compilation
    // this kind of calculation is accepted here
    const ONE_TIME_RESULT: u32 = 60 * 60;
    println!("Constant result of a one time calculation: {ONE_TIME_RESULT}");

    // SHADOWING
    // i.e. when a first variable is shadowed by a second one, which means that the second variable
    //      is what the compiler will see when you use the name of the variable
    // shadowing is scope-sensitive, which mean that a variable can be overshadowed and take
    //      different new values in different scopes
    // HOWEVER, variables remain immutable, because shadowing is different from modifying only the
    //      value of a variable. Indeed, we re-assign an entire variable instead.
    // SHADOWING WILL INVOKE A WARNING DURING COMPILATION
    let x = 1; // on purpose, '1' will never be printed, will provoke a warning at compile time
    let x = 2;
    // new scope
    {
        let x = 3;
        println!("The value in sub-scope is '{x}'"); // should be '3'
    }
    println!("The value in main scope is '{x}'"); // should be '2'
    //x = 3; // will not work though, because 'x' is immutable
    // affecting a new type is also permitted
    let x = "no error";
    println!("The value in main scope comes now with a new type : '{x}'");

    // ------------------------------
    // SCALAR TYPES
    // Rust is a statically typed language.
    // which means that it must know the types of all variables at compile time.
    // A scalar type represents a single value.
    // Rust has four primary scalar types:
    let a = 1; // integer
    let b = 0.1; // floating-point number
    let c = false; // boolean
    let d = 'd'; // character
    // The scalar types have a known size at compile time are stored entirely on the stack.

    // INTEGERS
    let mut x64: u64 = 0; // only works on 64bits machines
    let mut x: u8 = 255; // unsigned int coded on 8 bits so be careful with overflows
    //x+=1; // WILL PROVOKE AN INTEGER OVERFLOW ERROR
    // you can handle overflow behavior with different methods :
    //   - wrapping : will overflow by going back to '0' at maximum value
    //   - checked : will overflow by returning a None value. Good for 'if' tests.
    //   - overflowing : will overflow by returning a tuple with :
    //       (wrapping_add value, boolean 'true' if the number has overflowed).
    //     Good for if tests.
    //   - saturating : will overflow by returning the maximum value (like 255+1 = 255)
    // So you have the choice : let the code panic with overflows, or handle it with these methods.
    // These methods also work with other scalar tuto.functions than 'add'.
    let y: u8 = x.wrapping_add(1);
    println!("u8 255+1 with wrapping_add is equal to : {y}");
    let y:Option<u8> = x.checked_add(1); // 'Option<u8>' is 'None' or 'u8' type
    if y.is_none(){
        println!("u8 255+1 with checked_add is equal to : 'None'"); // will be printed
    }
    let y:(u8, bool) = x.overflowing_add(1); // will return (0, true)
    // if second value of the tuple is 'true'
    if y.1 {
        println!("u8 255+1 with overflowing_add has overflowed and is now : {}", y.0);
    }
    let y:u8 = x.saturating_add(1);
    println!("u8 255+1 with saturating_add is equal to : {y}");

    // FLOATING POINTS
    // All floating-point types are signed.
    // Only contains f32 or f64 (default).
    // The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is
    // capable of more precision.
    // represented according to the IEEE-754 standard
    let f1 = 0.1;
    let f2: f32 = 0.1;

    // BOOLEAN
    let b: bool = true;
    if b {
        println!("The condition was true.")
    }

    // CHARACTER
    let c = 'c';
    // since it is UTF-8 encoded, emojis are supported
    let c: char = '🚀';
    println!("Rocket science ! {c}");

    // COMPOUND TYPES
    // They can group multiple values into one type.
    // Rust has two primitive compound types:
    //   - tuples
    //   - arrays (and vectors which is a subtype)

    // TUPLE
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tuple: (u32, char, bool) = (0, 'a', false);
    println!("Tuple values are : '({},{},{})'", tuple.0, tuple.1, tuple.2);
    // A pattern with 'let' to take the tuple and turn it into separate variables is called
    // 'destructuring'.
    let (x, y, z) = tuple; // destructuring
    println!("First value of tuple : {x}");
    println!("Second value of tuple : {y}");
    println!("Third value of tuple : {z}");
    // The tuple without any values is called 'unit'.
    // This value and its corresponding type are both written () and represent an empty value or an
    // empty return type.
    // 'Expressions' implicitly return the unit value if they don’t return any other value.
    // Because everything has a type and a value in rust, even statements.
    let unit: () = ();
    //x; // this is an expression and is implicitly equal to a unit.

    // ARRAY
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays always have a fixed length.
    let array: [u32; 3] = [1, 2, 3]; // explicit declaration
    println!("First value of array : {}", array[0]);
    println!("Second value of array : {}", array[1]);
    println!("Third value of array : {}", array[2]);
    let array2 = [4, 5, 6]; // implicit declaration
    // initialize an array with 10 zeros
    let only_zeros = [0; 10];
    print!("Initialized an array with : [");
    for x in only_zeros { print!("{x},")}; print!("]");
    println!();

    // STRING
    // 'String' type and string literals 'str' are heap-based data, but they behave differently.
    // 'str' is an immutable sequence of UTF-8 bytes of dynamic length somewhere in the heap.
    //   Use it when you do not need to own or modify your string data.
    //   Since the size is unknown, you can only handle it behind a pointer. They are fast and
    //   efficient because we know the contents at compile time, so the text is hardcoded directly
    //   into the final executable.
    // 'String' is a mutable heap string type, like 'Vec': use it when you need to own or modify
    //   your string data. It allows to allocate an amount of memory in the heap, unknown at compile
    //   time, to hold the contents. Then String dereferences to a &str view of the String's data.
    //   It is made of 3 parts :
    //     - a pointer to the memory that holds the content
    //     - a length : how much the contents of the string is currently using, in bytes
    //     - a capacity : total amount of the usable memory for the string, in bytes.
    //          The allocator fixes it.
    // IN OTHER WORDS : USE 'str' FOR IMMUTABLE DATA AND 'String' FOR MUTABLE ONES TO BE OWNED.
    let s1: String = String::from("hello");
    let s2: &str = "world";
    println!("'String' type: {}", s1);
    println!("'str' type: {}", s2);

    // The following expression explains what happens when you create a 'String' from a 'str'.
    // The double colon :: operator allows us to namespace the variable. This operator says that
    //   the memory must be requested from the memory allocator at runtime and that we need to
    //   return this memory to the allocator when we’re done with our String. This is done when the
    //   code goes out of the scope of this variable (eg: at the end of the function).
    // This kind of string can be mutated afterward, even if it is less efficient than the literals.
    // The stack view remains the same in the heap, but we use another location in the heap to
    // allocate an extended String :
    // MORE INFORMATION ABOUT OWNERSHIP IN ../ownership/main.rs
    let immutable_str: &str = "hello";
    let mut s = String::from(immutable_str);
    s.push_str(", world !"); // appends a literal to a String.
    println!("Mutated String view of the stack : '{}'", s); // but the stack view remains the same
    println!("Stack view didn't changed : '{}'", immutable_str);

    // FOR MORE INFORMATION ABOUT STRING
    // Read :
    //   - ../../../ownership.md
    //   - ../ownership/main.rs

    // SLICES
    // READ OWNERSHIP TUTORIAL BEFORE LEARNING ABOUT SLICES
    // Slices let you reference a contiguous sequence of elements in a collection rather than the
    // whole collection.
    // A slice is a kind of reference, so it does not have ownership : it borrows the elements from
    // the initial collection.
    // A slice is noted [starting_index..ending_index] where :
    //   - starting_index is the first position in the slice
    //   - ending_index is ONE MORE THAT THE LAST POSITION IN THE SLICE !

    // STRING SLICES
    // Since slices are references, a String Slice is an immutable reference to a 'str' type.
    // It means that a String Slice variable only contains a pointer and a length :
    //   - The pointer is equal to the address of the byte starting the substring
    //   - The length is equal to the length of the substring
    // String slices are a good way to deal with substrings.
    // Example of String Slices :
    let s = String::from("hello world");
    let hello :&str = &s[..5]; // slice references the 4 first elements "hello" (0 to 4th)
    let world :&str = &s[6..]; // slice references the 5 last elements "world" (6th to 11th)
    println!("Frist slice is '{hello}'"); // move hello into println
    println!("Second slice is '{world}'"); // move world into println
    // 's' is still available in the scope !

    // INTEGER SLICE
    let x = [1, 2, 3, 4, 5];
    let slice :&[i32] = &x[1..3]; // 2nd to 3rd element [2, 3]
    assert_eq!(slice, &[2, 3]);

    // TIPS ON SLICE
    // It sometimes useful to use a Slice instead of an array of elements.
    // Because it allows to use them as reference instead of arrays of values, avoiding moves.
    // Example :
    let my_string = String::from("hello world !");
    let my_slice = &my_string[..]; // this slice contains all the elements of the original string
    println!("The whole string is '{my_slice}'"); // borrows the string instead of moving it into println
    let first_word = first_word(my_slice); // borrows the slice into the function
    println!("The first word is : '{first_word}'"); // moves first_word into println



    // VECTOR
    // Similar collection type as arrays (same library) that is allowed to grow or shrink in size.
    // TODO



}

// first_word finds the first words in a string if the elements are separated by a whitespace
// this function takes a slice as an argument, because it is more convenient
// because strings are easily convertible into slices
fn first_word(rs: &str) -> &str {
    let bytes = rs.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // if a whitespace is found
            return &rs[..i]; // returns the slice of the substring before the whitespace
        }
    }

    &rs[..] // otherwise, there is no whitespace -> return the whole slice
}
