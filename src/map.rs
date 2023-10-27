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
    let mut map = Map::new(0, 0);
    print(&map);
}
