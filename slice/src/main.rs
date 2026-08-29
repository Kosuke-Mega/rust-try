fn main() {
    let mut s: String = String::from("hello world");
    let world = first_world(&s);

    s.clear();
    println!("the first world is: {}", world);
}

fn first_world(s: &String) -> &str {
    let bytes:&[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i[0..i];
        }
    }

    s.len()
}
