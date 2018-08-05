extern crate ggez;
extern crate rand;
extern crate sha2;

#[macro_use]
extern crate arrayref;

mod level;
mod room;
mod seed;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use level::{create_level, Level};

struct MainState {
    level: Level,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            level: create_level(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.level.update(ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.level.draw(ctx)?;

        

       

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("Dungeon", "Lapz", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();


    use std::fs::File;
    use std::io::Write;


   let mut file = File::create("map.txt").expect("Couldn't create file");

   file.write(format!("{:#?}",state.level.corridors).as_bytes()).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
