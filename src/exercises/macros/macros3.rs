// Source: https://stackoverflow.com/a/62267586

// Variant 1: New way (Rust 2018)

use macros::my_macro;

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // TODO: Does this also export `my_macro` outside of this crate?
    //     If so, that might be undesirable...
    pub(crate) use my_macro;
}

fn main() {
    my_macro!();
}

// Variant 2: Old way

// #[macro_use]
// mod macros {
//     macro_rules! my_macro {
//         () => {
//             println!("Check out my macro!");
//         };
//     }
// }

// fn main() {
//     my_macro!();
// }
