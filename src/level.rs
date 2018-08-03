use rand::{StdRng,Rng,self,SeedableRng};
use room::Room;
use std::fmt::{self, Display};

const MAX_ROOMS: i32 = 10;
const MIN_ROOM_WIDTH: i32 = 4;
const MIN_ROOM_HEIGHT: i32 = 5;
const MAX_ROOM_WIDTH: i32 = 8;
const MAX_ROOM_HEIGHT: i32 = 12;

#[derive(Debug)]
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<i32>>,
    rooms: Vec<Room>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let board = (0..height).map(|_| vec![0; width as usize]).collect();
        Level {
            width,
            height,
            board,
            rooms: vec![],
        }
    }

    pub fn place_rooms(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rng = StdRng::from_rng(&mut rng).unwrap();

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

                self.board[y][x] = 1;
            }
        }

        self.rooms.push(*room);
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{:?} ", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
