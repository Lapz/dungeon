extern crate rand;

mod level;
mod room;

use level::Level;

fn main() {
   
    let mut level = Level::new(48, 40);

    level.place_rooms();
    println!("{}", level);
}
