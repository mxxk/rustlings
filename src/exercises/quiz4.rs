// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz!

macro_rules! my_macro {
    ("world!") => {"Hello world!"};
    ("goodbye!") => {"Hello goodbye!"};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}