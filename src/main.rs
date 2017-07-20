extern crate rand;

mod generation;

use generation::names::{Culture};

fn main() {
    for _ in 0..10 {
        let culture = Culture::new();
        println!("Culture {}", culture);
        println!("Culture name {}:", culture.generate_name());
        for _ in 0..20 {
            println!("\t{} {}", culture.generate_name(), culture.generate_name())
        }
    }
}
