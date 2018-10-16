
pub fn encode(mut s: String) -> String {
    let base64_elems: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();
    let mut output: String = "".to_string();
    let mut padding: String = "".to_string();
    let pad_count = s.len() % 3;

    if pad_count > 0 {
        padding = "=".repeat(pad_count);
        s += &"0".repeat(pad_count);
    }

    let chars: Vec<u32> = s.chars().map(|n| n as u32).collect();

    for c in (0..s.len()).step_by(3) {

        if c > 0 && (c / 3 * 4) % 76 == 0 {
            output += &"\r\n".to_string();
        }

        let mut n = chars[c] << 16;
        if (c+1) < chars.len() {
            n += chars[c+1] << 8;
        }
        if (c+2) < chars.len() {
            n += chars[c+2] << 8;
        }
        let mut m = vec![(n >> 18) & 63, (n >> 12) & 63, (n >> 6) & 63, n & 63];
        output += &format!(
            "{}{}{}{}",
           base64_elems[m[0] as usize],
           base64_elems[m[1] as usize],
           base64_elems[m[2] as usize],
           base64_elems[m[3] as usize]
        );
    }
    output = output[0..(output.len() - pad_count)].to_string();
    format!("{}{}", output, padding)
}
