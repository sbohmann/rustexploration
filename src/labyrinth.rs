use crate::map::Map;

struct Labyrinth {
    map: Map,
    solver: Solver,
    player: Player
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

struct Solver {
    map: Map
}

struct Player {
    map: Map
}

impl Player {
    fn move_(&self) {
        self.map.set_width(987);
    }
}
