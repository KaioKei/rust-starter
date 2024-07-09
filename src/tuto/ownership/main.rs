use std::ffi::OsString;

//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
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
    let s1 = String::from("hello");
    let s2 = s1;
    // SO THERE IS A PROBLEM :
    // When Rust will call the 'drop' function when 's1' and 's2' are getting out of scope,
    // Rust will try to free the same memory location twice.
    // Therefore, Rust implements a security that invalidate 's1' after 'let s2 = s1;'.
    // In this case, Rust does not have to free 's1' anymore, but it is also unusable in this scope.
    // This process in Rust is called 'move'. We say that 's1' was 'moved'.
    // For example, the following code does not compile because 's1' was moved to 's2' :
    //println!("{s1} world!"); // does not compile because s1 was invalidated (moved) by Rust

    // CLONING
    // to deeply copy the heap data of the String, not just the stack data, we can use a
    // method called "clone".
    // In this example, the heap data DOES get copied.
    // It means that a new pointer is created for 's2', and its length and capacity are inherited from 's1'.
    // Finally, the data of 's1' is copied to the location referenced by the pointer of 's2' in the heap.
    // Cloning is more expensive than simply copying the variable in memory consumption perspective,
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
    // Just like other variables, references are immutable :
    let s1 = String::from("hello");
    let s2 = String::from(", world !");
    let r1 = &s1; // works as expected
    //r1 = &s2; // does not work because r1 is immutable
    // let's use a mutable reference :
    let mut r2 = &s1;
    r2 = &s2; // works because r2 was declared mutable
    println!("Printing a string through its mutable reference: {r2}");

    // IMMUTABLE VAR OF A REFERENCE VS IMMUTABLE REFERENCE VS IMMUTABLE VAR
    let si = String::from("immutable");
    let mut sm = String::from("mutable");
    // this is an immutable variable of a immutable reference to an immutable variable, i.e. you :
    //   - cannot change 'r' to point somewhere else than 'si' (immutable)
    //   - cannot change the 'si' value using this reference 'r' (immutable)
    //   - cannot change the 'si' value (immutable)
    let r = &si;
    // this is an immutable variable of an immutable reference to a mutable variable, i.e. you :
    //   - cannot change 'r' to point somewhere else than 'sm' (immutable)
    //   - cannot change the 'si' value using this reference 'r' (immutable)
    //   - can change the 'sm' value (mutable) and you cannot do it using this reference anyway
    let r = &sm;
    // this is a mutable variable of an immutable reference to a mutable variable, i.e. you :
    //   - can change 'r' to point somewhere else than 'sm' (mutable)
    //   - cannot change the 'sm' value using this reference 'r' (immutable)
    //   - can change the 'sm' value (mutable) but you cannot do it using this reference
    let mut r = &sm;
    // this is an immutable variable of a mutable reference to a mutable variable, i.e. you :
    //   - cannot change 'r' to point somewhere else than 'sm' (immutable)
    //   - can change the 'sm' value using this reference 'r' (mutable)
    //   - can change the 'sm' value (mutable) and you can do it using this reference
    let r = &mut sm;
    // this a mutable variable of a mutable reference to a mutable variable, i.e. you :
    //   - can change 'r' to point somewhere else than 'sm' (mutable)
    //   - can change the 'sm' value using this reference 'r' (mutable)
    //   - can change the 'sm' value (mutable) and you can do it using this reference
    let mut r = &mut sm;
    // this is a mutable variable of an immutable reference to an immutable variable, i.e. you :
    //   - can change 'r' to point somewhere else than 'si' (mutable)
    //   - cannot change the 'si' value using this reference 'r' (immutable)
    //   - cannot change the 'si' value (mutable) and you cannot do it using this reference anyway
    let mut r = &si;
    // NOT POSSIBLE : this is an immutable variable of a mutable reference to an immutable variable
    // You cannot use a mutable reference to an immutable variable
    // otherwise it would assume that you can modify an immutable variable -> nonsense to Rust
    // let r = &mut si; // impossible
    // let mut r = &mut si; // impossible

    // the following example modifies a mutable string, using a mutable reference.
    // Borrowing the string into the 'modify' function induces 2 things :
    //   1. Since we have to modify the string, the string MUST BE mutable
    //   2. String we use borrowing, the reference MUST BE mutable
    let mut s = String::from("hello");
    modify(&mut s);
    println!("The string was modified using its reference : {s}");

    // !! WARNING !!
    // YOU CANNOT HAVE MORE THAN ONE REFERENCE TO A MUTABLE VALUE.
    // The following example raises an error :
    //let r1 = &s2;
    //let r2= &s2;





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