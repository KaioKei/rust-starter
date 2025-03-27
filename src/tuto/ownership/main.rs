use std::ffi::OsString;

//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
//https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    println!("Ownership tutorial.");

    // ---------------------------
    // OWNERSHIP, SCOPE AND MEMORY
    // ---------------------------

    // SCOPE
    // the variable 's' is valid as long as it is in the current scope, i.e. from the moment it is
    //  declared to the end of the scope.
    // When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put the code to return
    // the memory. Rust calls drop automatically at the closing curly bracket.
    let s = "hello";

    // FIRST READ HOW STRING WORK IN :
    //  - ../variables/main.rs

    // HEAP VS STACK
    // Scalar types have a known size at compile time are stored entirely on the stack.
    // The following example pushes a byte array onto the stack, and then gets a view of that data
    // as a &str using the heap :
    use std::str;
    // bytes pushed onto the stack
    let x: [u8; 5] = [b'h', b'e', b'l', b'l', b'o']; // 'hello' byte array
    // immutable str using the heap (5 bytes allocated) for each byte pushed onto the stack
    let stack_str: &str = str::from_utf8(&x).unwrap();
    println!("view of the stack using the heap : '{}'", stack_str); // viewing the heap

    // HANDLE THE HEAP WITH MEMORY REQUESTS
    // The following expression explains what happens when you create a 'String' from a 'str'.
    // The double colon :: operator allows us to namespace the variable. This operator says that
    // the memory must be requested from the memory allocator at runtime and that we need to
    // return this memory to the allocator when we’re done with our String. This is done when the
    // code goes out of the scope of this variable (eg: at the end of the function).
    // This kind of string can be mutated afterward, even if it is less efficient than the literals.
    // The stack view remains the same in the heap, but we use another location in the heap to
    // allocate an extended String :
    let immutable_str: &str = "hello";
    let mut s = String::from(immutable_str); // double colon :: requests memory at runtime to be owned in this scope
    s.push_str(", world !"); // appends a literal to a String.
    println!("Mutated String view of the stack : '{}'", s); // but the stack view remains the same
    println!("Stack view didn't changed : '{}'", immutable_str);

    // HEAP COPY
    // The following expression explains what happens when you copy a 'String'.
    // Remember that a String is made of 3 parts : a pointer, a length and a capacity.
    // In this example, the pointer, the length and the capacity are copied.
    // It means that the pointers of 's1' and 's2' are the same, so they point to the same location
    // in the heap.
    // THE DATA ITSELF IS NOT COPIED !!!
    // In other words, 's1' and 's2' are 2 different objects on top of the stack
    // ('s2' on top of 's1'), but point to the same data in the heap :
    //   - address_s1 = address_s2
    //   - length_s1 = length_s2
    //   - capacity_s1 = capacity_s2
    //   - s1 != s2 on the stack !
    let s1 = String::from("hello");
    let s2 = s1;
    // SO THERE IS A PROBLEM :
    // When Rust will call the 'drop' function when 's1' and 's2' are getting out of scope,
    // Rust will try to free the same memory location twice.
    // Therefore, Rust implements a security that invalidate 's1' after 'let s2 = s1;', by
    // implementing the feature 'copy' to heap-based variables.
    // In other words, Rust :
    //   - use 'copy' from 's1' to 's2'
    //   - invalidates 's1' in this scope
    // In this case, Rust does not have to free 's1' anymore, but it is also unusable in this scope.
    // This process in Rust is called 'move'. We say that 's1' was 'moved'.
    // For example, the following code does not compile because 's1' was moved to 's2' :
    //println!("{s1} world!"); // does not compile because s1 was invalidated (moved) by Rust

    // CLONING
    // to deeply copy the heap data of the String, not just the stack data, we can use a
    // method called "clone".
    // In this example, the heap data DOES get copied.
    // It means that a new pointer is created for 's2', and its length and capacity are inherited
    // from 's1'.
    // Finally, the data of 's1' is copied to the location referenced by the pointer of 's2' in the
    // heap.
    // Clone is more expensive than copy of the variable, in memory consumption perspective,
    // because it involves more operations from the memory allocator and more memory heap usage
    let s1 = String::from("hello");
    let s2 = s1.clone(); // cloning s1 to s2
    println!("{s1} world !"); // is working because s2 is a clone of s1, without s1 being moved to s2
    println!("{s2} world again !");

    // STACK-ONLY COPY
    // In this example, x IS NOT moved into y. But it is copied and pushed onto the stack :
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // This kind of copy concern scalar types, which have a known size at compile time and are
    // stored entirely on the stack.
    // One exception : copy also concerns Tuples if they only implement scalar types. Example :
    //   - `(i32, i32)` implements 'copy'
    //   - `(i32, String)` does not implement 'copy'
    // Copies of scalar types are quick to make.
    // That means there’s no reason Rust would want to prevent x from being valid after the copy into y.
    // In other words, copying or cloning scalar types are similar.
    // Rust do not allow the 'copy' trait for a type that implements the 'drop' trait (compile-time error).
    // In the following example, we can see that the String variable is moved into the function,
    // but not the integer because 'drop' is not a trait of i32 type (scalar) :
    let s = String::from("hello");
    this_fn_takes_ownership(s); // 's' is moved into the function ...
    // ... and 's' is no longer available in this scope : 's' was moved.
    //println!("{s}"); // So this statement will provoke a compile-time error
    let x = 5;
    this_fn_makes_copy(x); // 'x' IS NOT moved into this function, but is copied into the function,
    // because i32 has not the 'drop' trait (scalar type).
    // so 'x' is still available in this scope.
    println!("x is still available and is equal to {x}"); // so this statement works

    // RETURN VALUES
    // returning a value transfers the ownership to the upward scope.
    // This following example calls a function that creates a string and returns it.
    // So the ownership of the string is transferred to the actual scope.
    // In other words, the string is moved from the inside of the function to the outside scope
    // calling it :
    let s1 = get_string_ownership();
    println!("Get the ownership of the string '{s1}'");
    // This following example creates a string.
    // Then the string is moved into a function. But the function returns the same string.
    // So the ownership is given back to this scope :
    let s2 = String::from("mine");
    let s2 = takes_and_gives_back_ownership(s2);
    println!("Gave back the ownership of the string: '{s2}'");
    // There is a way to handle a variable in the same scope while without transfering ownership.
    // This is called 'references'

    // ---------------------------
    // REFERENCES AND BORROWING
    // ---------------------------
    // REFERENCE
    // A reference is a specific kind of variable, containing only an address of another variable.
    // A 'reference' is like a pointer : it’s an address to access the data stored at that address,
    // and that data is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type
    // for the life of that reference.
    // For example, a 'reference' of a string is a variable only holding the pointer part of the
    // string, composed with a pointer, a length and a capacity.
    // In the following example ;
    // - 's1' is a variable of type 'String'
    // - 'r1' is a variable of type '&String' (reference type to a string)
    let s1 = String::from("hello");
    let r1 = &s1;
    println!("Printing the string using its reference: {r1}");

    // BORROWING
    // Transferring the reference of a variable into another scope is called 'borrowing'.
    // In the following example, '&s1' is the reference of 's1' but it does not own 's1'.
    // So the ownership of s1 is not transferred into the function by calling the reference.
    // In other word, 's1' IS NOT moved into the function, 's1' IS borrowed to the function.
    let s1 = String::from("hello");
    this_fn_doesnt_take_ownership(&s1); // call function by reference, s1 is not moved, but 'borrowed'
    println!("So I can still use this string in the current scope : {s1}"); // s1 still valid in this context

    // MUTABLE REFERENCES
    // They are expressed as '&mut' and points to a type of variable that is mutable.
    // the following example modifies a mutable string, using a mutable reference.
    // Borrowing the string into the 'modify' function induces 2 things :
    //   1. Since we have to modify the string, the string MUST BE mutable
    //   2. String we use borrowing, the reference MUST BE mutable
    let mut s :String = String::from("hello");
    modify(&mut s);
    println!("The string was modified using its reference : {s}");

    // WARNING : a mutable reference can be held by an immutable variable.
    // Example :
    let mut s :String = String::from("hello");
    let r :&mut String = &mut s; // 'r' is immutable but hold the mutable reference of 's'
    // Indeed, you can hold an immutable String in a mutable reference.
    // It means that the variable cannot hold anything else than this mutable reference.
    // For example, the following raises an error :
    //r = &s2;
    // In the other hand, you can have a mutable variable of an immutable reference.
    // Example :
    let mut r2 :&String = &s1; // 'r2' is mutable, but not 's1' pointed by '&s1'
    r2 = &s2; // works because r2 was declared mutable. 's2' is also immutable.

    // IMMUTABLE VAR OF REFERENCE VS IMMUTABLE REFERENCE VS IMMUTABLE VAR
    let si :String = String::from("immutable");
    let mut sm :String = String::from("mutable");
    // this is an immutable variable of an immutable reference to an immutable variable, i.e. you :
    //   - cannot change 'r' for something else than the reference '&si' (immutable)
    //   - cannot change the 'si' value using this reference ('&' immutable)
    //   - cannot change the 'si' value (immutable)
    let r :&String = &si;
    // this is an immutable variable of an immutable reference to a mutable variable, i.e. you :
    //   - cannot change 'r' for something else than the reference '&sm' (immutable)
    //   - cannot change the 'si' value using this reference ('&' immutable)
    //   - can change the 'sm' value (mutable) but you cannot do it using this reference
    let r :&String = &sm;
    // this is a mutable variable of an immutable reference to a mutable variable, i.e. you :
    //   - can change 'r' for something else than the reference '&sm' (mutable)
    //   - cannot change the 'sm' value using this reference ('&' immutable)
    //   - can change the 'sm' value (mutable) but you cannot do it using this reference
    let mut r :&String = &sm;
    // this is an immutable variable of a mutable reference to a mutable variable, i.e. you :
    //   - cannot change 'r' for something else than the reference '&sm' (immutable)
    //   - can change the 'sm' value using this reference ('&mut' mutable)
    //   - can change the 'sm' value (mutable) and you can do it using this reference
    let r :&mut String = &mut sm;
    // this a mutable variable of a mutable reference to a mutable variable, i.e. you :
    //   - can change 'r' for something else than the reference '&sm' (mutable)
    //   - can change the 'sm' value using this reference ('&mut' mutable)
    //   - can change the 'sm' value (mutable) and you can do it using this reference
    let mut r :&mut String = &mut sm;
    // this is a mutable variable of an immutable reference to an immutable variable, i.e. you :
    //   - can change 'r' for something else than the reference '&si' (mutable)
    //   - cannot change the 'si' value using this reference ('&' immutable)
    //   - cannot change the 'si' value (mutable) and you cannot do it using this reference
    let mut r :&String = &si;
    // NOT POSSIBLE : this is an immutable variable of a mutable reference to an immutable variable
    // You cannot use a mutable reference to an immutable variable
    // otherwise it would assume that you can modify an immutable variable -> nonsense to Rust
    // let r :&mut String = &mut si; // impossible
    // let mut r :&mut String = &mut si; // impossible

    // !! WARNING !!
    // A MUTABLE REFERENCE CANNOT BE BORROWED TWICE IN THE SAME SCOPE
    // A MUTABLE REFERENCE CANNOT BE BORROWED IF AN IMMUTABLE REFERENCE OF THE SAME VAR IS HELD BY ANOTHER VARIABLE
    // It is because Rust prevents data races at compile time when :
    //   - Two or more pointers access the same data at the same time.
    //   - At least one of the pointers is being used to write to the data.
    //   - There’s no mechanism being used to synchronize access to the data.
    // For example, the following example raises an error :
    //let r1 :&mut String = &mut s2;
    //let r2 :&mut String = &mut s2; // same mutable reference held by another variable = error
    // For example, the following also raises an error :
    //let r1 :&String = &s;
    //let r2 :&mut String = &mut s; // same reference but mutable, already held by r1 = error
    // Otherwise, immutable references can be held by more than one variable :
    let r1 :&String = &s;
    let r2 :&String = &s; // same reference held by another variable = OK, because it is read-only
    // The following works because references 'r1' and 'r2' are dropped by the 'println' function :
    println!("{r1} and {r2}");
    let r3 :&mut String = &mut s;

    // DANGLING REFERENCE
    // A Dangling reference is a reference where the data goes out of scope before the reference.
    // It is dangerous : it suggests that the data is out of scope while its reference is owned.
    // Rust prevents dangling references at compile-time, so it will raise an error.
    // Example of a function that creates dangling reference :
    let dangling_reference :&String = dangle(); // the reference is borrowed here
    fn dangle() -> &String {
       let s = String::from("hello");
       &s
    } // here the data of 's' is dropped, but not its reference, borrowed in the upper scope -> error

} // end of the main function scope. All variables are no longer valid and their memory are returned to the allocator.

fn this_fn_takes_ownership(some_string: String) { // 'some_string' comes into this function's scope
    println!("took ownership of String '{some_string}'");
} // 'some_string' goes out of this function's scope -> 'drop' trait is called and the memory is freed.
// 'some_string' becomes inaccessible in other scopes calling this function

fn this_fn_makes_copy(some_integer: i32) {
    println!("did not too ownership of i32 '{some_integer}'") // 'some_integer' comes into this function's scope
} // 'some_integer' goes out of this function's scope -> 'drop' trait IS NOT called because of the i32 type, which is stack-only based, so the value remains onto the stack.
// 'some_integer' remains accessible in other scopes calling this function.

// get_string_ownership will move the returned value into the scope that calls it
fn get_string_ownership() -> String {
    // this string is returned and moves out to the calling scope
    String::from("yours")
}

// takes_and_gives_back_ownership takes a string in arguments
// this string is moved into the function's scope.
// but this function also returns it, so the string ownership is given back to the upward scope.
fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

// this function accepts as an argument only references of a string.
// It means that it manipulates the pointer of a string instead of a string entire type, containing
// the pointer plus a length and a capacity.
// The advantage is that the ownership of the variable concerned by the pointer is not transferred
// into this function, and the string remains usable in the upward scope.
fn this_fn_doesnt_take_ownership(s: &String) {
    println!("Using this string reference does not transfer ownership of string : {s}");
} // s goes out of scope (dropped), but not the variable concerned by the address in the reference


fn modify(s: &mut String) {
    s.push_str(", world !");
}
