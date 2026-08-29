use std::ops::Add;

assert_eq!(4.125f32.add(5.75), 9.875);
assert_eq!(10.add(20, 10 + 20));

trait Add<Rhs = Self> {

}
