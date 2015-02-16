//! Top-level doccy

#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

fn main(){
//    fn1();
//    fn2();
//    tuples();
//    structs();
//    ordering_match();
//    enum_match();
//    arrays();
    stdin()
}

fn stdin(){
    #[feature(io)]
    use std::old_io;

    print!("Type something: ");
    let input = old_io::stdin()
        .read_line()
        .ok()
        .expect("Failed to read line");

    println!("You said {}", input)
}

fn arrays(){
    let a = [1,2,3];
    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!["{}", e];  // [] or () for macros, innit.
    }
}

fn enum_match(){
    enum OptInt {
        Some(i32),
        Nonez
    }
    let x = OptInt::Some(4);
    let y = OptInt::Nonez;

    println!("The value is {}", match x {

//        OptInt::None => "missing"  <-- mismatch string/static string
        OptInt::Nonez => "missing".to_string(), // :(


//        OptInt::Some(n) => n,  <-- type mismatch n/"missing"
        OptInt::Some(n) => n.to_string(),


    });

}

fn ordering_match(){
    use std::cmp::Ordering::{self, Less, Equal, Greater};

    fn cmp(a: i32, b:i32) -> Ordering {
        if a < b { Less }
        else if b < a { Greater }
        else { Equal }
    }

    let (a,b) = (3,2);
    println!("cmp({}, {}) => {}", a, b, match cmp(a,b) {
        Less => "Less",
        Greater => "Greater",
        Equal => "Equal",
    });
}

fn structs(){
    // a struct with one member
    // aka a newtype
    struct Inches(i32);

    let length = Inches(10);

    println!("{} inches", { let Inches(l) = length; l });
    // TODO: This is ugly. Can we destructure into the
    // result of an expression more betterer?
}

fn tuples(){
    let mut x = (2,3);
    x = (4,5);
    println!("x = ({}, {})", x.0, x.1);

    let y = (x,x);
    println!("y.0.0 = {}", (y.0).0);
}

/// `fn2` is a function which I'm using to check the
/// return value of an assignment (ie what is the value
/// of `aa` ?)
///
/// Please note:
///   * the return type of the assignment `bb = 1` is `()`
///   * so the value of `aa` is `()`
///   * we can't print `()` as we can't coerce it to a string
///   * in fact, I don't know anything you _can_ do with a `()`
///
/// Also, this is rustdoc, written in markdown with a `///`.
/// (I don't know how to generate the docs yet though :( )
fn fn2(){
    let aa = { let bb = 1; };    
    println!("aa={}", if aa == () {"unit"} else {"value"})
}

fn fn1(){
    let (x,y) = (1,2);
    let z = { if x == y { 5 } else { 10 } };
    println!("z={}", z);
}

