use rand::{StdRng,Rng,SeedableRng};
use room::Room;
use std::fmt::{self, Display};
use seed::create_hash;
use ggez::{Context, GameResult};

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
}

#[derive(Debug,Clone)]
enum Tile {
    Empty,
    Walkable
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let board = (0..height).map(|_| vec![Tile::Empty; width as usize]).collect();
        Level {
            width,
            height,
            board,
            rooms: vec![],
        }
    }

    pub fn update(&mut self,ctx:&mut Context) -> GameResult<()> {
        for room in self.rooms.iter() {
            room.draw(ctx)?;
        }

        Ok(())
    }


    pub fn draw(&self,ctx:&mut Context) -> GameResult<()> {

        for room in self.rooms.iter() {
            room.draw(ctx)?;
        }

        Ok(())
    }

    pub fn place_rooms(&mut self,rng:&mut StdRng) {

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

            let room = Room::new(x, y, width, height);

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

    fn add_room(&mut self,room:&Room) {
    
         for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;

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
            Tile::Empty => write!(f,""),
            Tile::Walkable => write!(f,"1")
        }
    }
}


pub fn create_level() -> Level {
   let hash = create_hash("It's time to go");

   let seed = array_ref!(hash.as_bytes(),0,32);


   let mut rng:StdRng = SeedableRng::from_seed(*seed);
   
   let mut level = Level::new(720, 400);

    level.place_rooms(&mut rng);
    level
}