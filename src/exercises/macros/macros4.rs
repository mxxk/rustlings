macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

pub fn main() {
    my_macro!();
    my_macro!(7777);
}
