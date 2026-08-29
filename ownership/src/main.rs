fn main() {
    let _s1: String = String::from("hello");
    let (_s2, len) = calculate_length(_s1);

    println!("The length of '{}' is {}.", _s2, len);
}

fn calculate_length(_s: String) -> (String, usize) {
    let length:usize = _s.len();

    (_s, length)
}


