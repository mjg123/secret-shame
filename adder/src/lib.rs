//! The `adder` crate blah blah blah
//!
//! # Examples
//!
//! ```rust
//! assert_eq!(5, adder::add_two(3));
//! ```


#![allow(unstable)]
// needed because the homebrew version of rust is out of date?

#[test]
fn it_works() {
    assert!(true)
}

#[test]
#[should_fail(expected = "assertion failed")] // substring match
fn it_doesnt() {
    assert_eq!("yolo", "ninja")
}



/// The `add_two` fn blah blah blah
///
/// # Examples
///
/// ```rust
/// use adder::add_two;
/// assert_eq!(6, add_two(4));
/// ```
pub fn add_two(x:i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::add_two;
    use self::test::Bencher;
    
    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench] //mark
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(4));
    }

}
