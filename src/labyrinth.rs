use std::cell::RefCell;
use std::rc::Rc;
use crate::map::Map;

type Ref<T> = Rc<RefCell<T>>;

fn new<T>(value: T) -> Ref<T> {
    return Rc::new(RefCell::new(value))
}

struct Labyrinth {
    map: Ref<Map>,
    player: Ref<Player>,
    solver: Ref<Solver>
}

impl Labyrinth {
    fn new() -> Labyrinth {
        let map = new(Map::new());
        let player = new(Player { map: map.clone() });
        let solver = new(Solver { map: map.clone(), player: player.clone()});
        return Labyrinth {
            map,
            player,
            solver
        };
    }

    fn solve(&self) {

    }
}

struct Solver {
    map: Ref<Map>,
    player: Ref<Player>
}

struct Player {
    map: Ref<Map>
}

impl Player {
    fn move_(&mut self) {
        self.map.borrow_mut().set_width(987);
    }
}

pub(crate) fn run() {
    let labyrinth = Labyrinth::new();
    labyrinth.solve()
}
