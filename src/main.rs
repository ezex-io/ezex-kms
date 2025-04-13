// greet returns a greeting message.
pub fn greet() -> String {
    "Hello, world!".to_owned()
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;
