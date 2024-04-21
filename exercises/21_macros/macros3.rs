// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// First method for my_macro!()
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    // "Newer Method" for macros::my_macro!()
    pub(crate) use my_macro;
}

fn main() {
    my_macro!();
    macros::my_macro!();
}
