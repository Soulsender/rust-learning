fn main() {
    
    let x: u8 = 9; // 0 - 255
    let y: i8 = 10; // -128 - 127
    // you cannot add types that are not the same
    //let z = x + y;

    let x: u8 = 255;
    let y: u8 = 1;
    // you cannot add these together due to overflow
    //let c = a + b;

    let num1: u8 = 255;
    let num2: u8 = 10;
    let final_num = num1 / num2;
    println!("{}", final_num); 

    // adding different types
    let x = 34u8;
    let x = 12_u8;
    let y = 64 as i64; 
    let z = (x as i64) + y;
    println!("{}", z);



}
