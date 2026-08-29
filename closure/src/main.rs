fn main() {
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    assert_eq!(is_even(14), true);
}
