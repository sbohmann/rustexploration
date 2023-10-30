use Field::*;

pub(crate) struct Map {
    width: i16,
    height: i16,
    data: Vec<Field>
}

impl Map {
    pub fn new(width: i16, height: i16) -> Map {
        Map {
            width,
            height,
            // data: vec![Field::Empty; usize::try_from(width * height).unwrap()]
            // Yes, this is very, very ugly but adding a map loader has simply not yet been done 😎
            data: vec!(Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
                       Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Goal, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                       Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall)
        }
    }

    pub fn get(&self, x: i16, y: i16) -> Field {
        return self.data[
            usize::try_from(y * self.width + x)
                .ok()
                .unwrap()];
    }

    pub fn free(&self, x: i16, y: i16) -> bool {
        return
            x >= 0 &&
            x >= 0 &&
            x < self.width &&
            y < self.height &&
            self.get(x, y) != Wall;
    }

    pub fn print(&self, player_x: i16, player_y: i16) {
        (0..5).for_each(|_| println!());
        for y in 0..self.height {
            for x in 0..self.width {
                if x == player_x && y == player_y {
                    print!("**")
                } else {
                    let letter = match self.get(x, y) {
                        Empty => "  ",
                        Wall => "██",
                        Goal => "░░"
                    };
                    print!("{letter}")
                }
            }
            println!()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Field {
    Empty,
    Wall,
    Goal
}
