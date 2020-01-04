fn main() {
    test_01();
    test_02();
    test_03();
    test_04();
    test_05();
    test_06();
    test_07();
}

fn test_01() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is invalidated due to move operation in assignment.
    // move = (shallow copy && invalidate source)
    //println!("{}, world!", s1); 
    println!("{}, world!", s2);
}

fn test_02() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // clone is equivalent to deep copy.  Both src and dest remain valid
    println!("s1 = {}, s2 = {}", s1, s2);    
}

fn test_03() {
    let x = 5;
    let y = x;
    
    // values known at compile time are not subject to invalidation
    // any group of simple scalar values can be Copy, and nothing that 
    // requires allocation or is some form of resource is Copy. 
    println!("x = {}, y = {}", x, y);

}

fn test_04() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
    



fn test_05() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}



fn test_06() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);  // function returns tuple

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // tuples can return multiple values! Nice!!!
}





fn test_07() {
    let s1 = String::from("hello");

    // pass in a referrence to variable.  
    // - referrence = borrow
    // - similar to const pointer...
    // https://stackoverflow.com/questions/2336230/difference-between-const-pointer-and-reference
    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}


// function parameter type is now referrence
fn calculate_length2(s: &String) -> usize {
    s.len()
}
