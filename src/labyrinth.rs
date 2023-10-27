use std::rc::Rc;
use crate::map::Map;

struct Labyrinth {
    map: Rc<Map>,
    player: Rc<Player>,
    solver: Rc<Solver>
}

impl Labyrinth {
    fn new() -> Labyrinth {
        let map = Rc::new(Map::new());
        let player = Rc::new(Player { map: map.clone() });
        let solver = Rc::new(Solver { map: map.clone(), player: player.clone()});
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
    map: Rc<Map>,
    player: Rc<Player>
}

struct Player {
    map: Rc<Map>
}

impl Player {
    fn move_(&self) {
        self.map.set_width(987);
    }
}

fn run() {
    let labyrinth = Labyrinth::new();
    labyrinth.solve()
}
