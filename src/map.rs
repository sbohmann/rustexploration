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
            data: vec!()
        }
    }

    pub fn get(&self, x: i16, y: i16) -> Field {
        return self.data[
            usize::try_from(y * self.width + x).ok().unwrap()];
    }

    pub fn free(&self, x: i16, y: i16) -> bool {
        return
            x >= 0 &&
            x >= 0 &&
            x < self.width &&
            y < self.height &&
            self.get(x, y) == Empty;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Field {
    Empty,
    Wall
}
