// some languages use a "garbage collector" which is just a program that goes and looks for memory that is no longer being used (ie. Python)
// some languages must be explicitly told when to allocate and remove memory (ie. C)
// rust uses ownership, which will stop the compiler from compiling the program if the rules of ownership are broken.


/// OWNERSHIP RULES
/// - all values have an owner
/// - there can only be one owner
/// - when owner goes out of scope, value is dropped

fn main() {
    // "s" is a string literal where the value is hardcoded
    // var is valid until the end of the scope
    let s = "hello";

    // String type can store an amount that is unknown at compile time
    let z = String::from("hello");
    // :: namespaces "from" under "String" type
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print "hello, world!"

    // STACK
    // faster than the heap

    // HEAP
    // slower than the stack
    // requires a pointer

    let s1 = String::from("hello");
    let s2 = s1;
    // when s1 is assigned to s2, the String data is copied (the pointer, length, and capacity on the stack)
    // we do not copy the data on the heap that the pointer refers to

}
// "s" is now out of the scope