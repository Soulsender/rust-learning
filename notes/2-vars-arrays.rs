fn main() {

    // constants cannot change once made
    // you cannot reassign constants
    // constants are usually UPPER_CASE
    const CONSTANT_VARIABLE: u32 = 60;
    println!("constant var: {}", CONSTANT_VARIABLE);

    // you cannot reassign a variable by default - immutable
    let x = 4;
    println!("x is: {}", x);

    // you can add the "mut" keyword to make the variable mutable
    let mut y = 5;
    println!("y is: {}", y);
    y = 8;
    println!("y is now: {}", y);

    // if you don't want the var to be mutable, you can just redefine it
    let z = 10;
    println!("z is: {}", z);
    let z = 20;
    println!("z is now: {}", z);
    
    // this is a scope
    // interior scope does not effect outside of scope
    {
        // you can also use the outside of the scope in the scope
        let x = x - 2;
        println!("x is: {}", x);
    }

    // ARRAYS
    {
        // arrays are like tuples, but only one datatype is allowed
        // you cannot reassign data unless mutable
        let array = [1, 2, 3, 4, 5];
        
    }    

    // TUPLES
    {
        // explicit tuple
        let tuple: (i32, bool, char) = (1, true, 's');

        // you cannot directly print an entire tuple
        //println!("{}", tuple);

        // you can print objects inside a tuple
        println!("{}", tuple.2);

        // you can make tuples mutable
        let mut tuple: (i16, char, u32) = (4, 'c', 6);
        println!("{}", tuple.0);
        tuple.0 = 32;
        println!("{}", tuple.0);

    }

    // INTEGERS
    {
        // define type "integer32"
        let x: i32 = 2;
        // implicit type "integer32"
        let x = 2;

        // define type "unsigned integer32"
        // cannot be a negative number
        let y: u32 = 823; 
    }
    
    // FLOATS
    {
        let y: f32 = 10.9;
        let y: f64 = 12.5;
    }

    // BOOLEANS
    {
        // define "true" and "1" booleans
        // 1 is true
        let z: bool = true;
        //let z: bool = 1;

        // define "false" and "0" booleans
        // 0 is false
        let z: bool = false;
        //let z: bool = 0;
    }

    // CHARACTERS
    {
        // define char variable
        let letter: char = 'a';
    }

}
