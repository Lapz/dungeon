use ggez::graphics;
use ggez::{Context, GameResult};
use rand::{Rng, SeedableRng, StdRng};
use room::Room;
use seed::create_hash;
use std::fmt::{self, Display};

const MAX_ROOMS: i32 = 10;
const MIN_ROOM_WIDTH: i32 = 40;
const MIN_ROOM_HEIGHT: i32 = 50;
const MAX_ROOM_WIDTH: i32 = 80;
const MAX_ROOM_HEIGHT: i32 = 120;

#[derive(Debug)]
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<Tile>>,
    rooms: Vec<Room>,
    corriders: Vec<graphics::Rect>,
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Walkable,
}

impl Tile {
    pub fn draw(&self, ctx: &mut Context, x: f32, y: f32) -> GameResult<()> {
        match *self {
            Tile::Empty => graphics::ellipse(
                ctx,
                graphics::DrawMode::Fill,
                graphics::Point2::new(x, y),
                0.0,
                0.0,
                1.0,
            ),
            Tile::Walkable => {
                let rect = graphics::Rect::new(x, y, 1.0, 1.0);
                graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)
            }
        }
    }
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let board = (0..height)
            .map(|_| vec![Tile::Empty; width as usize])
            .collect();
        Level {
            width,
            height,
            board,
            rooms: vec![],
            corriders: vec![],
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for room in self.rooms.iter() {
            room.draw(ctx)?;
        }

        for c in self.corriders.iter() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, *c)?;
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for room in self.rooms.iter() {
            room.draw(ctx)?;
        }

        for c in self.corriders.iter() {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, *c)?;
        }

        Ok(())
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        for _ in 0..MAX_ROOMS {
            let mut x = rng.gen_range(0, self.width);
            let mut y = rng.gen_range(0, self.height);

            let width = rng.gen_range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH);
            let height = rng.gen_range(MIN_ROOM_HEIGHT, MAX_ROOM_HEIGHT);

            if x + width > self.width {
                x = self.width - width;
            }

            if y + height > self.height {
                y = self.height - height;
            }

            let mut collides = false;

            let room = Room::new(x as f32, y as f32, width as f32, height as f32);

            for other_room in self.rooms.iter() {
                if room.intersect(other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.add_room(&room);
            }
        }
    }

    pub fn place_corrider(&mut self, rng: &mut StdRng) {
        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let other = self.rooms[i + 1];

            match rng.gen_range(0, 2) {
                0 => {
                    let rect = match room.centre.x < other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    };

                    self.corriders.push(rect);

                    let rect = match room.centre.y < other.centre.y {
                        true => self.vert_corrider(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corrider(other.centre.y, room.centre.y, other.centre.x),
                    };

                    self.corriders.push(rect);
                }
                _ => {
                    let rect = match room.centre.y <= other.centre.y {
                        true => self.vert_corrider(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corrider(other.centre.y, room.centre.y, other.centre.x),
                    };

                    self.corriders.push(rect);

                    let rect = match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    };

                    self.corriders.push(rect);
                }
            };
        }
    }

    fn horz_corridor(&mut self, start_x: f32, end_x: f32, y: f32) -> graphics::Rect {
        graphics::Rect::new(start_x, y, end_x - start_x, 10.0)
        // for col in start_x..end_x + 1 {
        //     self.board[y as usize][col as usize] = Tile::Walkable;
        // }

        // unimplemented!()
    }

    fn vert_corrider(&mut self, start_y: f32, end_y: f32, x: f32) -> graphics::Rect {
        graphics::Rect::new(x, start_y, 10.0, end_y - start_y)
    }

    fn add_room(&mut self, room: &Room) {
        for row in 0..(room.height as i32) {
            for col in 0..(room.width as i32) {
                let y = (room.y + row as f32) as usize;
                let x = (room.x + col as f32) as usize;

                self.board[y][x] = Tile::Walkable;
            }
        }

        self.rooms.push(*room);
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{} ", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::Empty => write!(f, ""),
            Tile::Walkable => write!(f, "1"),
        }
    }
}

pub fn create_level() -> Level {
    let hash = create_hash("manuelneuersweeperkeeper");

    let seed = array_ref!(hash.as_bytes(), 0, 32);

    let mut rng: StdRng = SeedableRng::from_seed(*seed);

    let mut level = Level::new(720, 400);

    level.place_rooms(&mut rng);
    level.place_corrider(&mut rng);
    level
}
