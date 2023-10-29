use crate::map::Field::Empty;

pub(crate)  struct Map {
    width: i16,
    height: i16,
    data: Vec<Field>
}

impl Map {
    pub fn new(width: i16, height: i16) -> Map {
        Map {
            width,
            height,
            data: vec![Field::Empty; usize::try_from(width * height).unwrap()]
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
            self.get(x, y) == Empty;
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let letter = match self.get(x, y) {
                    Empty => ' ',
                    Field::Wall => '#'
                };
                print!("{letter}")
            }
            println!()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Field {
    Empty,
    Wall
}
