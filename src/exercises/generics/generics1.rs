// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

fn main() {
    // TODO: What is the default lifetime of the &str ref here? 'static?
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
