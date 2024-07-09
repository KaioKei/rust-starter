# Ownership

Source : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

Ownership is a set of rules that govern how a Rust program manages memory.

Some languages have garbage collection; other languages must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.

## Stack vs Heap

Stack and Heap have 2 very different memory usage approaches.

**The stack** stores values as *last in, first out* (pile). All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. This process is called *pushing onto the stack*.

**The heap** allocates data. It uses a memory allocator that finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. This process is called *allocating*.

Pushing to the stack or getting from the stack is **faster** than allocating on or getting from the heap, because the allocator never has to search for a place to store new data or retrieve allocated data; that location is always at the top of the stack.

**During runtime**, when your code calls a function, the arguments (including pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

## Variables in memory

### Primitives

Rust has simple types called *Primitive types*. They are :

- Boolean
- Numeric : integer and float
- Textual : `char` and literal `str`
- Never : `!` which is a type with no value

```rust
let x = 5;
let y = x;
```

Since `x` and `y` are simple scalar types (integers), they are both pushed onto the stack. Bind the value `5` to `x`; then make a copy of the value of `x` and bind it to `y`.

## Ownership Rules

**Ownership addresses:**

- Keeping track of what parts of code are using what data on the heap
- minimizing the amount of duplicate data on the heap
- cleaning up unused data on the heap, so you don’t run out of space are all problems that

Its rules are :

- Each value in Rust has an owner.
- Only one owner at a time per value.
- When the owner goes out of scope, the value will be dropped.

Follow this code examples to learn ownership : [ownership tuto main.rs](src/tuto/ownership/main.rs)

