#![allow(unstable)]

extern crate adder;

#[test]
fn it_works_in_a_dir(){
    assert_eq!(4, adder::add_two(2));
}

