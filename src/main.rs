// Many functions in this crate will not be called, as the point of the exercise
// was to get them to compile. Thus, disable the dead code lint.
#![allow(dead_code)]

fn main() {}

mod exercises {
    mod quiz1;
    mod quiz2;
    mod quiz3;
    mod quiz4;
    mod variables {
        mod variables1;
        mod variables2;
        mod variables3;
        mod variables4;
        mod variables5;
        mod variables6;
    }
    mod functions {
        mod functions1;
        mod functions2;
        mod functions3;
        mod functions4;
        mod functions5;
    }
    mod r#if {
        mod if1;
        mod if2;
    }
    mod move_semantics {
        mod move_semantics1;
        mod move_semantics2;
        mod move_semantics3;
        mod move_semantics4;
        mod move_semantics5;
    }
    mod primitive_types {
        mod primitive_types1;
        mod primitive_types2;
        mod primitive_types3;
        mod primitive_types4;
        mod primitive_types5;
        mod primitive_types6;
    }
    mod structs {
        mod structs1;
        mod structs2;
        mod structs3;
    }
    mod enums {
        mod enums1;
        mod enums2;
        mod enums3;
    }
    mod modules {
        mod modules1;
        mod modules2;
        mod modules3;
    }
    mod collections {
        mod vec1;
        mod vec2;
        mod hashmap1;
        mod hashmap2;
    }
    mod strings {
        mod strings1;
        mod strings2;
    }
    mod error_handling {
        mod errors1;
        mod errors2;
        mod errors3;
        mod errors4;
        mod errors5;
        mod errors6;
    }
    mod generics {
        mod generics1;
        mod generics2;
        mod generics3;
    }
    mod r#option {
        mod option1;
        mod option2;
        mod option3;
    }
    mod traits {
        mod traits1;
        mod traits2;
    }
    mod tests {
        mod tests1;
        mod tests2;
        mod tests3;
    }
    mod standard_library_types {
        mod arc1;
        mod box1;
        mod iterators1;
        mod iterators2;
        mod iterators3;
        mod iterators4;
        mod iterators5;
    }
    mod threads {
        mod threads1;
    }
    mod macros {
        mod macros1;
        mod macros2;
        mod macros3;
        mod macros4;
    }
    mod clippy {
        mod clippy1;
        mod clippy2;
    }
    mod conversions {
        mod as_ref_mut;
        mod from_into;
        mod from_str;
        mod try_from_into;
        mod using_as;
    }
}
