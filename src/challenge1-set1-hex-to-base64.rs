/**
* Pass input like: challenge1-set1-hex-to-base64 <hex string>
* e.g.: challenge1-set1-hex-to-base64 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
**/
use std::env;
use std::iter::FromIterator;

#[macro_use]
mod common;

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


    let mut v: Vec<char> = [].to_vec();
    for c in (0..input_string.len()).step_by(2) {
        let val: &str = input_string.get(c..(c+2)).unwrap();
        v.push(u8::from_str_radix(val, 16).map(|n| n as char).unwrap())
    }

    println!("{}", String::from_iter(v));
}
