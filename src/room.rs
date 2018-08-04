use ggez::graphics;
use ggez::{Context, GameResult};
#[derive(Debug, Copy, Clone)]
pub struct Room {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub centre: graphics::Point2,
}

// impl Into<graphics::Rect> for Room {
//     fn into(self) -> graphics::Rect {
//          graphics::Rect::new(self.x ,self.y , self.width , self.height)
//     }
// }

impl From<Room> for graphics::Rect {
    fn from(room: Room) -> Self {
        graphics::Rect::new(room.x, room.y, room.width, room.height)
    }
}
impl<'a> From<&'a Room> for graphics::Rect {
    fn from(room: &Room) -> Self {
        graphics::Rect::new(room.x, room.y, room.width, room.height)
    }
}

impl Room {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Room {
            x,
            y,
            width,
            height,
            centre: graphics::Point2::new(x + (width / 2.0), y + (height / 2.0)),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);

        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        Ok(())
    }

    pub fn intersect(&self, other: &Room) -> bool {
        // let r =self.into();
        let r1: graphics::Rect = self.into();
        r1.overlaps(&other.into())

        // unimplemented!()
    }
}
