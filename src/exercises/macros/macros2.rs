// Needs to be defined *before* where it's used.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

pub fn main() {
    my_macro!();
}

