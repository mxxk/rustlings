#![allow(dead_code)]

const NUMBER: i32 = 3;
fn main() {
    // NOTE: Rust won't infer the type of a const item, even if only
    //     `i32` would work given the usage below.
    constrain_const_type(NUMBER);
    println!("Number {}", NUMBER);
}

fn constrain_const_type(_num: i32) { }