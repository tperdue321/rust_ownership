// exploring ownership and references in Rust
// Travis Perdue
// taken almost verbatum from
// https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html 

fn main() {
    // EXPLORE OWNERSHIP BASICS

      // string is not valid here it has not yet been declared.
    {
        // must be an immutable string
        // put directly into the binary
        let string = "hello"; // string is valid from here forward
        // do stuff with string
    } // string is no longer valid. the prog has left that scope

    {
        // can be a mutable string
        // requests memory from the OS
        let mut s = String::from("hello"); // string saved on the heap
        s.push_str(", world."); // append a literal to a String
        println!("{}", s); // "hello, world."
    } // Rust calls Drop here when scope is gone to free memory

    // bind 5 to x  
    let x = 5; 
    // make a copy
    let y = x;
    // x isn't invalidated because int's size is known at compile time
    // can be put on the stack because of it
    // therefore shallow/deep copies are the same
    println!("x = {}, y = {}", x, y);

    // makes ptr to string on heap with len and capacity values as well
    let s1 = String::from("string");

    // copies pointer to string on heap, not the string itself.
    // known as a move and invalidates s1
    // difference between shallow and deep copies
    let s2 = s1;

    // use of moved value will not work
    // println!("{}", s1); // this will not compile

    let s1 = String::from("string");
    let s2 = s1.clone();
    // this is valid
    println!("s1 = {}, s2 = {}", s1, s2);


    // direct from the book (just great explaining comments)
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.


    // println!("{}", s); // WON'T WORK 
    println!("{}", x); // Copied value. will work


    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.

    let (s4, len) = calculate_length(s3);  // pass ownership and return it after
                                            // doing calculations

    // this works, but is a little tedious/ridiculous to always pass
    // ownership around so much.
    // let's explore references
    println!("The length of \"{}\" is {}.", s4, len);



    // EXPLORE BORROWING AND REFERENCES
    let s1 = String::from("hello");
    let len = calculate_len_with_ref(&s1);
    println!("The length of \"{}\" is {}.", s4, len);

    let mut s2 = String::from("hellur");
    change(&mut s2);
    println!("{}", s2);

    // can only have one mutable reference a scope to prevent
    // data race
    let r1 = &mut s2;
    // this will not work because of these 3 rules
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.
    // let r2 = &mut s2;

    let mut s3 = String::from("hellur");
    {
        let r3 = &mut s3;
    }

    let r4 = &mut s3;

    let mut s = String::from("hello s");
    let r1 = &s; // immutable ref
    let r2 = &s; // second because as many refs can read the same data safely

    // not allowed with immutable refs existing
    // let r3 = &mut s; // BAD. no one with an immutable ref wants the data to
    // change unexpectedly

    // impossible with these rules to create dangling references/pointers

    // let's try
    // compiler will stop this from compiling
    // let ref_to_dangler = dangler();


    // EXPLORE SLICES

    let s = String::from("  hello world.");
    let string_literal: &str = "some string literal";
    // references to a slice of a string
    let hello = &s[2..7];
    let world = &s[8..13];
    println!("{}", hello);
    println!("{}", world);

    // if referring to beginning or end of a collection,
    // the first/last value can be left off the range (..)

    let slice = &s[0..2];   // same
    let slice = &s[..2];    // same

    let len = s.len(); // 1 past last index of string

    let slice = &s[2..len]; // same
    let slice = &s[2..];    // same

    let slice = &s[0..len]; // same
    let slice = &s[..];     // same

    // this won't work. it needs to be a slice (type &str)
    // let first_word: &str = first_word(&s);

    let word = first_word(&string_literal[..]);
    println!("first word => {}", word);
    let word = first_word(string_literal);
    println!("first word => {}", word);
    let word = first_word(&s[..]);
    println!("first word => {}", word);

    // s.clear(); // ERROR. because first_word references s, clearing s causes
    // compile time error. 

    // another type of slice
    let a = [1,2,3,4,5];
    let slice = &a[0..2];
}   // from EXPLORE OWNERSHIP BASICS section
    // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.

// direct from the book (just great explaining comments)
fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it.

    let some_string = String::from("hello");    // some_string comes into scope.

    some_string                                 // some_string is returned and
                                                // moves out to the calling
                                                // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope
    a_string  // a_string is returned and moves out to the calling function.
}

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}

// having references as func params is called BORROWING
fn calculate_len_with_ref(string: &String) -> usize {
    string.len()
}   // string goes out of scope but is a reference so nothing happens to what
    // it points to


// borrowed values can't be modified by default
// must be a mutable value and be passed to func
// as a mutable ref
fn change(string: &mut String) {
    string.push_str(", world");
}

// this will break if attempted to compile
// fn dangler() -> &String {   // dangle returns a reference to a String
//     let s = String::from("heyo");   // s is a new String
//     &s  // we return a reference to the String, s
// }       // Here, s goes out of scope, and is dropped. Its memory goes away.
        // the reference to s now points to null and breaks
        // Rust compiler stops us from this


fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    // variables for finding first char in string
    // while ignoring leading whitespace.
    let mut char_found = false;
    let mut first_char: usize = 0;

    for(i, &item) in bytes.iter().enumerate() {
        // first char found after any leading white space
        if !char_found && !(item == b' ') {
            char_found = true;
            first_char = i;
        }
        // first space after first word ignoring any
        // leading white space
        else if item == b' ' && char_found {
            return &string[first_char..i];
        } // end if/else if
    } // end for
    // entire string has no whitespace
    &string[..]
} // end first_word


