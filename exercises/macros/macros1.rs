// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! mymacro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    mymacro!();
}
