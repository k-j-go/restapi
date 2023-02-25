use std::fmt;
use std::fmt::Formatter;


#[test]
fn test_generic() {
    fn foo<T>(x: T) -> T
        where T: std::ops::Mul<Output=T> + Copy
    {
        x * x
    }

    let r = foo::<u32>(2_u32);
    println!("{}", r);
}

