use std::cell::RefCell;
use std::rc::Rc;
use crate::map::Map;

struct Labyrinth {
    map: Rc<RefCell<Map>>,
    player: Rc<RefCell<Player>>,
    solver: Rc<RefCell<Solver>>
}

impl Labyrinth {
    fn new() -> Labyrinth {
        let map = Rc::new(RefCell::new(Map::new()));
        let player = Rc::new(RefCell::new(Player { map: map.clone() }));
        let solver = Rc::new(RefCell::new(Solver { map: map.clone(), player: player.clone()}));
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
    map: Rc<RefCell<Map>>,
    player: Rc<RefCell<Player>>
}

struct Player {
    map: Rc<RefCell<Map>>
}

impl Player {
    fn move_(&mut self) {
        self.map.borrow_mut().set_width(987);
    }
}

fn run() {
    let labyrinth = Labyrinth::new();
    labyrinth.solve()
}
