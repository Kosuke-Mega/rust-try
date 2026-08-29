use std::io;

fn main() {
    let _sum = 5 + 10;
    let _defference: f32 = 95.5 - 4.3;
    let _product: u32 = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    let _remainder = 43 % 5;

    let mut _t: bool = true;
    _t = false;

    let mut _c: char = 'z';
    _c = 'Z';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;

    println!("The value of y is: {_y}");

    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "Octorber", "November", "December"];

    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    let _first = _b[0];
    let _second = _b[1];

    let mut _index = String::new();

    io::stdin().read_line(&mut _index).expect("Failed to read line");

    let _index: usize = _index.trim().parse().expect("Index entered was not a number");

    let _element = _b[_index];

    println!("The value of the element at index {_index} is: {_element}");
}
