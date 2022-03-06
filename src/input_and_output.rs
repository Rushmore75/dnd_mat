use std::io;

pub fn get_input(msg: String, input: &mut String) {

    /*
    You have to parse into your own value, making it do this automatically
    is too much work for the pay off. Maybe later...
    */

    println!("{}", msg);
    // let mut input = String::new();
    io::stdin().read_line(input).expect("Failed to read input");
    // trim()
    
    
}