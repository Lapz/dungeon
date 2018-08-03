extern crate rand;

extern crate sha2;

#[macro_use]
extern crate arrayref;

mod level;
mod room;
mod seed;

use level::Level;
use seed::create_hash;
use rand::{StdRng,SeedableRng};

fn main() {
   
   let hash = create_hash("It's time to go");

   let seed = array_ref!(hash.as_bytes(),0,32);


   let mut rng:StdRng = SeedableRng::from_seed(*seed);
   
   let mut level = Level::new(48, 40);

    level.place_rooms(&mut rng);
    println!("{}", level);
}
