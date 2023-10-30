use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use rand::random;
use crate::labyrinth::Direction::{Down, Left, Right, Up};
use crate::map::{Field, Map};

type Ref<T> = Rc<RefCell<T>>;

fn new<T>(value: T) -> Ref<T> {
    return Rc::new(RefCell::new(value))
}

struct Labyrinth {
    map: Ref<Map>,
    player: Ref<Player>,
    solver: Ref<dyn Solver>
}

impl Labyrinth {
    fn new(create_solver: fn(Ref<Map>, Ref<Player>) -> Ref<dyn Solver>) -> Labyrinth {
        let map = new(Map::new(10, 10));
        let player = new(Player { x: 8, y: 8, map: map.clone()});
        let solver = create_solver(map.clone(), player.clone());
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
        let [x, y] = self.new_position_for_direction(direction);
        return self.map.borrow().free(x, y)
    }

    fn new_position_for_direction(&self, direction: Direction) -> [i16; 2] {
        return match direction {
            Direction::Up => {[self.x, self.y - 1]}
            Direction::Down => {[self.x, self.y + 1]}
            Direction::Left => {[self.x - 1, self.y]}
            Direction::Right => {[self.x + 1, self.y]}
        };
    }

    fn move_if_possible(&mut self, direction: Direction) {
        let [x, y] = self.new_position_for_direction(direction);
        if self.map.borrow().free(x, y) {
            self.x = x;
            self.y = y;
        }
    }
}

trait Solver {
    fn solve(&mut self);
}

struct RandomSolver {
    map: Ref<Map>,
    player: Ref<Player>,
    number_of_moves: usize
}

impl Solver for RandomSolver {
    fn solve(&mut self) {
        loop {
            let direction = match random::<u8>() % 4 {
                 0 => Up,
                 1 => Down,
                 2 => Left,
                 3 => Right,
                _ => panic!("Logical error")
            };
            let mut player= self.player.borrow_mut();
            player.move_if_possible(direction);
            let map = self.map.borrow();
            map.print(player.x, player.y);
            self.number_of_moves += 1;
            let number_of_moves = self.number_of_moves;
            println!("{number_of_moves}");
            if map.get(player.x, player.y) == Field::Goal {
                return;
            }
            sleep(Duration::from_secs(1));
        }
    }
}

pub(crate) fn run() {
    let labyrinth = Labyrinth::new(
        |map: Ref<Map>, player: Ref<Player>| new(RandomSolver { map: map, player: player, number_of_moves: 0}));
    labyrinth.solve()
}
