/**
* Pass input like: challenge1-set1-hex-to-base64 <hex string>
**/
use std::env;
use std::process;
use std::iter::FromIterator;

fn _fail_exit(code: i32) {
    process::exit(code);
}

macro_rules! fail_exit {
    ($code:expr) => { _fail_exit($code) };
    () => { _fail_exit(-1) };
}

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
