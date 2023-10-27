pub struct Map {
    width: i16,
    height: i16,
    data: Vec<Field>
}

impl Map {
    pub fn new() -> Map {
        Map {
            width: 0,
            height: 0,
            data: vec!()
        }
    }

    pub fn set_width(&mut self, value: i16) {
        self.width = value;
        self.height = 0;
        self.data = vec!();
    }
}

pub(crate) fn print(instance: &Map) {
    let width = instance.width;
    println!("{width}")
}

enum Field {
    Empty,
    Wall
}

pub fn run() {
    let mut map = Map::new();
    map.set_width(987);
    print(&map);
}
