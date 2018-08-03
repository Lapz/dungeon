extern crate rand;
extern crate sha2;
extern crate ggez;

#[macro_use]
extern crate arrayref;

mod level;
mod room;
mod seed;

use level::{Level,create_level};
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};



struct MainState {
    level:Level,
}


impl MainState {
    fn new(ctx:&mut Context) -> GameResult<MainState> {
        Ok(MainState {
            level:create_level()
        })
    }
}


impl event::EventHandler for MainState {

    fn update(&mut self,ctx:&mut Context) -> GameResult<()> {

        self.level.update(ctx)?;

        Ok(())
    }


    fn draw(&mut self,ctx:&mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.level.draw(ctx)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
   
   let c =  conf::Conf::new();

   let ctx = &mut Context::load_from_conf("Dungeon","Lapz",c).unwrap();

   let state = &mut MainState::new(ctx).unwrap();

   if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

}


