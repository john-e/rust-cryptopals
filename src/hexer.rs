use std::iter::FromIterator;

pub fn hex_to_string(input_string: String) -> String {
    let mut v: Vec<char> = Vec::new();
    for c in (0..input_string.len()).step_by(2) {
        let val: &str = input_string.get(c..(c+2)).unwrap();
        v.push(u8::from_str_radix(val, 16).map(|n| n as char).unwrap())
    }
    return String::from_iter(v);
}
