use std::cell::{RefCell, RefMut};
use std::rc::Rc;

struct Labyrinth {
    map: Map,
    solver: Solver,
    player: Player,
}

impl Labyrinth {
    fn new(map: &Map) -> Labyrinth {
        Labyrinth {
            map: map.clone(),
            solver: Solver { map: map.clone() },
            player: Player { map: map.clone() },
        }
    }
}

macro_rules! unwrap {
    ($self:ident) => {
        (&mut *$self.data_reference.borrow_mut())
    }
}

#[derive(Clone)]
struct Map {
    data_reference: Rc<RefCell<MapData>>,
}

struct MapData {
    width: i16,
    height: i16,
    data: Vec<Field>,
}

impl Map {
    fn new() -> Map{
        Map {
            data_reference: Rc::new(RefCell::new(MapData {
                width: 0,
                height: 0,
                data: vec!()
            }))
        }
    }
    fn set_width(&self, value: i16) {
        let this = unwrap!(self);
        *this = MapData {
            width: value,
            height: 0,
            data: vec!()
        };
    }
    fn print(&self) {
        let width = unwrap!(self).width;
        println!("{width}")
    }
}

enum Field {
    Empty,
    Wall,
}

struct Solver {
    map: Map,
}

struct Player {
    map: Map,
}

impl Player {
    fn move_(&self) {
        self.map.set_width(987);
    }
}

pub fn run() {
    let map = Map::new();
    map.set_width(987);
    map.print();
}
