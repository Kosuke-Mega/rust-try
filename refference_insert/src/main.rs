
fn main() {
    // let r;
    // {
    //     let x = 2;
    //     r = &x;
    //     assert_eq!(*r, 2);
    //     println!("{}", *r);
    // }
    struct S<'a> {
        x: &'a i32,
        y: &'a i32
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S {x: &x, y: &y};
            r = s.x;
        }
    }
    println!("{}", r);
}
