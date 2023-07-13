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
    fn data(&self) -> RefMut<'_, MapData> {
        return self.data_reference.borrow_mut();
    }
    fn set_width(&self, value: i16) {
        (&mut *self.data()).width = value
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
    println!("Hi! Labyrinth")
}
