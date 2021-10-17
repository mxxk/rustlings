#![allow(dead_code)]

// Create an array with at least 100 elements in it where the ??? is.

fn main() {
    let a = [1; 101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}