fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 8;
    let z = &y; // a reference

    println!("z = {:p}", z);

    ///////////

    let x = Box::new(19);
    println!("{}", add_one(&*x));
        
}

fn add_one(x:&i32) -> i32 {
    *x + 1
}
