// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    (1) => {
        println!("another nicer slicer");
    }
}

fn main() {
    my_macro!();
    my_macro!(1);
}
