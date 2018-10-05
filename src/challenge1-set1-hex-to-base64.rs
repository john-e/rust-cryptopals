/**
* Pass input like: challenge1-set1-hex-to-base64 <hex string>
* e.g.: challenge1-set1-hex-to-base64 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
**/
use std::env;

#[macro_use]
mod common;
mod hexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("- Required an argument");
        fail_exit!();
    }

    let input_string: String = args.get(1).unwrap().to_string();

    if input_string.len() % 2 != 0 {
        println!("Length not valid");
        fail_exit!();
    }


    let str: String = hexer::hex_to_string(input_string);

    println!("{}", str);
}
