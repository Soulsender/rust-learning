use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    
    // "trim()" is escape char/newline
    // "parse()" gives integer if int is able to be converted to i64
    // "unwrap()" takes value and wraps into integer type
    let int_input: i64 = input.trim().parse().unwrap();
    
    println!("{}", int_input + 2);
}
