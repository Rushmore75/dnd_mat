use std::{io, str::FromStr};

/*
Normal, without message
*/
pub fn get_input() -> String {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read input");
    (*input.trim()).to_string()
}
pub fn get_input_parsed<F: FromStr>() -> Result<F, F::Err> {
    // basically just yoinked Rust's "parse" function 
    FromStr::from_str(&get_input())
}

/*
with message
 */
pub fn get_input_wmsg(msg: String) -> String {
    println!("{}", msg);
    get_input()
}
pub fn get_input_parsed_wmsg<F: FromStr>(msg: String) -> Result<F, F::Err> {
    // basically just yoinked Rust's "parse" function 
    FromStr::from_str(&get_input_wmsg(msg))
}




