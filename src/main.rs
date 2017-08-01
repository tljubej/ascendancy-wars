extern crate rand;

mod generation;
mod world;

use generation::names::Culture;
use world::World;

fn main() {
    // for _ in 0..10 {
    //     let culture = Culture::new();
    //     println!("Culture {}", culture);
    //     println!("Culture name {}:", culture.generate_name());
    //     for _ in 0..20 {
    //         println!("\t{} {}", culture.generate_name(), culture.generate_name())
    //     }
    // }

    let world = World::new(1000000, 10);

    // for person in world.people_by_power_level {
    //     println!("{}", person);
    // }
}
