use std::cell::{RefCell, RefMut};
use std::rc::Rc;

macro_rules! unwrap {
    ($self:ident) => {
        (&mut *$self.data_reference.borrow_mut())
    }
}

#[derive(Clone)]
pub struct Map {
    data_reference: Rc<RefCell<MapData>>,
}

struct MapData {
    width: i16,
    height: i16,
    data: Vec<Field>
}

impl Map {
    pub fn new() -> Map{
        Map {
            data_reference: Rc::new(RefCell::new(MapData {
                width: 0,
                height: 0,
                data: vec!()
            }))
        }
    }
    pub fn set_width(&self, value: i16) {
        let this = unwrap!(self);
        *this = MapData {
            width: value,
            height: 0,
            data: vec!()
        };
    }
    pub(crate) fn print(&self) {
        let width = unwrap!(self).width;
        println!("{width}")
    }
}

enum Field {
    Empty,
    Wall
}

pub fn run() {
    let map = Map::new();
    map.set_width(987);
    map.print();
}
