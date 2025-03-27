//https://doc.rust-lang.org/book/ch05-01-defining-structs.html
//https://doc.rust-lang.org/book/ch05-02-example-structs.html
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html

// Structure
// A Structure is composed of 'fields'.
// Each field has a name and a type.
// Then, a structure can be instantiated in a scope, giving values to the fields.
// Usually we use the owned 'String' type rather than the '&str' string slice type in structures.
// Because we want each instance of this struct to own all of its data and for that data to be valid
// for as long as the entire struct is valid.
// It’s also possible for structs to store references to data owned by something else, but to do so
// require the use of 'lifetimes', a Rust feature that we’ll discuss later.
struct User {
    // field_name: field_type
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    //token: &str // would require to use the 'lifetime' feature
}

// Tuple Structure
// Rust also supports structs that look similar to tuples, called tuple structs.
// They don’t have names associated with their fields; they just have the types of the fields.
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a
// different type from other tuples and when you don't have to name the fields.
// Unlike tuples, if two tuple structures share the same types and number of parameters, they are
// not considered equals by Rust.
// Examples :
struct Color(u8, u8, u8); // RGB from 0 to 255
struct Point(i32, i32, i32);
struct OtherPoint(i32, i32, i32); // Point and OtherPoint are different types

// Unit-Like Structure
// These structures behave similarly to '()', the unit type.
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have
// any data that you want to store in the type itself.
// For example, imagine that we implement behavior for this type such that every instance of
// SomeTrait is always equal to every instance of any other type,
// perhaps to have a known result for testing purposes.
struct SomeTrait;

// Methods & Associated functions
struct Rectangle {
    height: u32,
    width: u32,
}
// - 'Methods' are defined within the context of a struct and their first parameter is always
//   'self', which represents the instance of the struct the method is being called on.
// - 'Associated functions' are defined within an impl block and are associated with a type, but
//   they do not refer to self. Thus, they are used as commonly related to a type trait without
//   being related to a single instance. For example, are often used for constructors that will
//   return a new instance of the struct.
impl Rectangle {
    // METHODS
    // the first parameter type is always 'Self' or '&Self' and it references the type of the
    // structure it implements :
    //   - '&Self' will only borrow the structure, keeping it available afterward in the scope of an
    //     instantiated Rectangle. Usually, you will prefer to use this method definition in order
    //     to continue using the structure further in the code.
    //   - 'Self' will move the structure into its own method, invalidating it afterward in the
    //      scope of an instantiated Rectangle. this technique is usually used when the method
    //      transforms self into something else, and you want to prevent the caller from using the
    //      original instance after the transformation.
    // This method doesn't invalidate an instantiated rectangle after use :
    fn area_borrow(self: &Self) -> u32 {
        self.height * self.width
    }
    // a shorter syntax is accepted and remains the same as above once compiled :
    fn area_borrow_short(&self) -> u32 {
        self.height * self.width
    }
    // This method invalidates an instantiated rectangle after use :
    fn area_move(self: Self) -> u32 {
        self.height * self.width
    }
    // a shorter syntax is accepted and remains the same as above once compiled :
    fn area_move_short(self) -> u32 {
        self.height * self.width
    }

    // GETTERS
    // they are better than calling the field because it allows to implements more checks on
    // the fields
    fn width(self: &Self) -> u32 {
        if self.width > 0 {
            self.width
        } else {
            // stupid, but you can raise an error also
            0
        }
    }

    // MODIFYING
    // You can modify an instantiated structure with a mutable reference using '&Self'
    fn set_width(self: &mut Self, new_width: u32) {
        self.width = new_width;
    }
    fn set_height(self: &mut Self, new_height: u32) {
        self.height = new_height;
    }
    // shorter version
    fn set_height_short(&mut self, new_height: u32) {
        self.height = new_height;
    }

    // ASSOCIATED FUNCTIONS
    // new_square returns a new instance (Self) of Rectangle.
    // It is not a method since it doesn't refer to 'self' (as a first parameter), so it is
    // associated to the type 'Rectangle' and not to a particular instance.
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    println!("Structure tutorial");

    // Instantiate an immutable structure already defined.
    // Note that you cannot mark certain fields to mutable or immutable : or the entire structure
    // is immutable, or it is not.
    let user1 = User {
        // field_name: field_value
        active: false,
        username: String::from("Sauron"),
        email: String::from("sauron.eye@baraddur.com"),
        sign_in_count: 7,
    };
    println!("Sauron's email is : {}", user1.email);

    // Instantiate a mutable structure already defined
    let mut user2 = User {
        active: true,
        username: String::from("frodo"),
        email: String::from("frodo.baggins@shire.com"),
        sign_in_count: 1,
    };
    println!("Frodo's email is : {}", user2.email);

    // modifying an already instantiated structure.
    // the instance MUST BE mutable
    user2.email = String::from("frodo.baggins@valinor.com");
    println!("Frodo's email changed to : {}", user2.email);

    // Using a function to build a user
    let mut user3 = build_user(String::from("sam"),
                               String::from("sam.gamegie@shire.com"));
    println!("User3's username is : {}", user3.username);

    // Creating a structure from another one.
    let mut user3_bis = User {
        username: String::from("samwise"),
        // the following syntax tells to Rust to copy every other field of an already instantiated
        // struct into this one.
        // So it moves 'user3' into 'user3_bis' using 'copy' for each field.
        // user3 is then invalidated in the scope of the function main.
        // THE STRUCTURE HAS TO HAVE THE SAME DEFINITION (TYPE) !
        ..user3
    };

    // WARNING !
    // The structures are values AND NOT references.
    // It means that using a structure into another one to build it moves the original structure
    // into the new one.
    // In this case, the following statement raises an error because 'user3' was moved into
    // 'user3_bis' and is no longer available :
    //println!("{}", user3.email);

    // TUPLE STRUCTURES
    // Color and Point are 2 tuple structures :
    // they are tuples and structures but of different type, even if the 3 parameters would be of
    // the same type.
    let purple = Color(255, 0, 255);
    println!("Intensity of the red in 'purple': {}", purple.0);
    println!("Intensity of the green in 'purple': {}", purple.1);
    println!("Intensity of the blue in 'purple': {}", purple.2);
    let origin = Point(0, 0 ,0);
    let other_origin = OtherPoint(0, 0, 0); // origin != other_origin

    // UNIT-LIKE STRUCTURES
    let a_trait = SomeTrait;

    // IMPLEMENTED STRUCTURES
    // they can be implemented by methods.
    // Example :
    let r1 = Rectangle{width: 2, height: 3};
    let r1_borrowed_area = r1.area_borrow();
    println!("This rectangle is still valid in this scope and its area is: {}", r1_borrowed_area);
    let r1_moved_area = r1.area_move(); // this method invalidates r1 because it is moved into its own method
    println!("This rectangle is no longer valid in this scope but the result of area still is: {}", r1_moved_area);
    //println!("r1 is no longer available: {}", r1.width); // raises an error

    // GETTERS
    // getting a value with more checks
    let r1 = Rectangle{width: 2, height: 3};
    let r1_width = r1.width(); // the '()' tells Rust we use the getter instead of the field
    println!("This is the width using the getter: {}", r1_width);

    // MODIFYING
    let mut r2 = Rectangle{height: 4, width: 3};
    println!("Initial area is '{}'", r2.area_borrow());
    r2.set_height(1);
    r2.set_width(2);
    println!("New area is '{}'", r2.area_borrow());

    // MORE DETAILS
    // When you call a method with 'object1.something(object2)', Rust automatically adds '&',
    // '&mut', or '*' so 'object1' and 'object2' match the signature of the method.
    // In other words, the following are the same, because we defined the method as
    // 'set_height(self: &mut Self, new_height: u32)', so the '&mut r2' is automatically added by
    // Rust at compile time, in the place of 'r2' only :
    r2.set_height(1);
    (&mut r2).set_height(1);

    // ASSOCIATED FUNCTIONS
    // related to a type and not to an instance
    let square: Rectangle = Rectangle::new_square(2);
    println!("new square of size: {}x{}", square.height, square.width);
}

// This function builds a User structure.
// It returns a value, and not a reference, to avoid dangling references (see ownership tuto).
fn build_user(username: String, email: String) -> User {
    // implicit return
    User {
        active: true,
        // A shorthand definition can be used when the variable holding the value has the same name
        // as the field :
        username,
        email,
        sign_in_count: 1,
    }
}