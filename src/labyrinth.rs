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
        let map = new(Map::new(0, 0));
        let player = new(Player { x: 0, y: 0, map: map.clone()});
        let solver = new(Solver { map: map.clone(), player: player.clone()});
        return Labyrinth {
            map,
            player,
            solver
        };
    }

    fn solve(&self) {
        self.solver.borrow_mut().solve()
    }
}

struct Player {
    x: i16,
    y: i16,
    map: Ref<Map>
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Player {
    fn free(&self, direction: Direction) -> bool {
        let [x, y] = match direction {
            Direction::Up => {[self.x, self.y - 1]}
            Direction::Down => {[self.x, self.y + 1]}
            Direction::Left => {[self.x - 1, self.y]}
            Direction::Right => {[self.x + 1, self.y]}
        };
        return self.map.borrow().free(x, y)
    }
}

struct Solver {
    map: Ref<Map>,
    player: Ref<Player>
}

impl Solver {
    fn solve(&mut self) {

    }
}

pub(crate) fn run() {
    let labyrinth = Labyrinth::new();
    labyrinth.solve()
}
