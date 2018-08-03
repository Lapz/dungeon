use ggez::{Context, GameResult};
use ggez::graphics;
#[derive(Debug,Copy,Clone)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: (i32, i32),
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x,
            y,
            x2: x + width,
            y2: y + width,
            width,
            height,
            centre: (x + (width / 2), y + (height / 2)),
        }
    }

    pub fn draw(&self,ctx:&mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x as f32,self.y as f32, self.width as f32, self.height as f32);

        graphics::rectangle(ctx,graphics::DrawMode::Fill,rect)?;
        Ok(())
    }

    pub fn intersect(&self, other: &Room) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}
