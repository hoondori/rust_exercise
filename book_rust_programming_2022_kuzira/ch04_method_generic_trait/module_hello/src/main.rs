mod hello;

use hello::{world, korea};

fn main() {
    println!("{}", world::say());
    println!("{}", korea::say());
    println!("{}", crate::hello::korea::say_to_all());
}
