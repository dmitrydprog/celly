#![cfg(test)]
use traits::Cell;
use traits::Coord;
use traits::Grid;
use traits::Engine;
use traits::Consumer;
use engine::Sequential;
use grid::nhood::HexagonalNhood;
use grid::square::SquareGrid;

/// Implementation of [FHP model](https://en.wikipedia.org/wiki/FHP_model).
/// Assumes Hexagonal neighborhood.

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum Stage {
    Collision,
    Transport
}


impl Default for Stage {
    fn default() -> Self { Stage::Collision }
}

//      N
//   NW   NE
// W         E
//   SW   SE
//      S

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    NW = 0,
    NE = 1,
    W = 2,
    E = 3,
    SW = 4,
    SE = 5,
}


impl Direction {

    fn opposite(&self) -> Self {

        match *self {
            Direction::NW => Direction::SE,
            Direction::NE => Direction::SW,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
            Direction::SW => Direction::NE,
            Direction::SE => Direction::NW,
        }
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct FHP {
    particles: [bool; 6],
    stage: Stage,
    coord: (i32, i32)
}


impl Cell for FHP {
    type Coord = (i32, i32);

    fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>> {

        match self.stage {
            Stage::Collision => self.collision(neighbors),
            Stage::Transport => self.transport(neighbors),
        }
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        FHP {
            stage: Stage::Collision,
            coord: (coord.x(), coord.y()),
            ..Default::default()
        }
    }

    fn coord(&self) -> &Self::Coord {
        &self.coord
    }

    fn set_coord<C: Coord>(&mut self, coord: &C) {
        self.coord = (coord.x(), coord.y());
    }
}


impl FHP {

    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = FHP {
            stage: Stage::Transport,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) => {
                    let opposite = direction.opposite();
                    let head_on = self.particle(&direction) &&
                                  neighbor.particle(&opposite) &&
                                  !self.particle(&direction.perpendicular()) &&
                                  !neighbor.particle(&opposite.perpendicular());

                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                    }
                    else {
                        let particle = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, particle);
                    }
                },
                // Rebound
                None => {
                    if self.particle(&direction) {
                        let opposite = direction.opposite();
                        new.set_particle(&opposite, true);
                    }
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = FHP {
            stage: Stage::Collision,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {
                Some(neighbor) => {
                    let opposite = direction.opposite();

                    if neighbor.particle(&opposite) {
                        new.set_particle(&opposite, neighbor.particle(&opposite));
                    }
                },
                None => {
                    if self.particle(&direction) {
                        new.set_particle(&direction, true);
                    }
                }
            }
        }

        new
    }

    pub fn particle(&self, direction: &Direction) -> bool {
        self.particles[*direction as usize]
    }

    fn set_particle(&mut self, direction: &Direction, exists: bool) {
        let index = *direction as usize;
        self.particles[index] = exists;
    }

    #[inline]
    pub fn directions(&self) -> [Direction; 6] {
        [
            Direction::NW, Direction::NE, Direction::W,
            Direction::E, Direction::SW, Direction::SE
        ]
    }
}


// #[derive(Debug)]
// struct FHPRulesTestConsumer;

// use test_helpers::to_cell;


// impl FHPRulesTestConsumer {

//     pub fn new() -> Self {
//         FHPRulesTestConsumer { }
//     }

//     fn particles_count<C: Cell>(&self,
//                                  cells: &Vec<C>,
//                                  direction: &Direction) -> i32 {

//         cells.iter()
//              .map(|c| to_cell(c))
//              .filter(|c: &FHP| c.particle(direction) == true)
//              .count() as i32
//     }

//     fn find_cell<C: Cell>(&self,
//                           cells: &Vec<C>,
//                           x: i32, y: i32) -> FHP {

//         assert!(cells.iter().any(|c| c.coord().x() == x && c.coord().y() == y));

//         let found = cells.iter()
//                          .find(|c| c.coord().x() == x && c.coord().y() == y)
//                          .unwrap();

//         to_cell(found)
//     }

//     fn test_collision<G: Grid>(&self, grid: &G) {

//         let left_particle_count =
//             self.particles_count(&grid.cells(), &Direction::Left);
//         assert_eq!(left_particle_count, 0);

//         let right_particle_count =
//             self.particles_count(&grid.cells(), &Direction::Right);
//         assert_eq!(right_particle_count, 2);

//         let up_particles_count =
//             self.particles_count(&grid.cells(), &Direction::Up);
//         assert_eq!(up_particles_count, 1);

//         let down_particles_count =
//             self.particles_count(&grid.cells(), &Direction::Down);
//         assert_eq!(down_particles_count, 1);

//         let rebound_to_right =
//             self.find_cell(&grid.cells(), 0, 1);
//         assert_eq!(rebound_to_right.particle(&Direction::Right), true);

//         let head_on_up =
//             self.find_cell(&grid.cells(), 1, 0);
//         assert_eq!(head_on_up.particle(&Direction::Up), true);

//         let head_on_down =
//             self.find_cell(&grid.cells(), 2, 0);
//         assert_eq!(head_on_down.particle(&Direction::Down), true);
//     }

//     fn test_transport<G: Grid>(&self, grid: &G) {

//         let simple_move_to_right = self.find_cell(&grid.cells(), 1, 2);
//         assert_eq!(simple_move_to_right.particle(&Direction::Right), true);

//         let move_to_right_after_rebound = self.find_cell(&grid.cells(), 1, 2);
//         assert_eq!(move_to_right_after_rebound.particle(&Direction::Right), true);

//         let move_to_down_after_head_on = self.find_cell(&grid.cells(), 2, 1);
//         assert_eq!(move_to_down_after_head_on.particle(&Direction::Down), true);

//         let fixed_to_up_after_head_on = self.find_cell(&grid.cells(), 1, 0);
//         assert_eq!(fixed_to_up_after_head_on.particle(&Direction::Up), true);
//     }

//     fn pretty_print<G: Grid>(&self, grid: &G) {
//         println!("");

//         for y in 0 .. 3 {
//             print!("|");

//             for x in 0 .. 3 {
//                 let cell = self.find_cell(grid.cells(), x, y);
//                 let directions = cell.directions();
//                 let maybe_particle =
//                     directions.iter().find(|d| cell.particle(d));

//                 let to_print = match maybe_particle {
//                     Some(&Direction::Up) => " ^ |",
//                     Some(&Direction::Left) => " < |",
//                     Some(&Direction::Right) => " > |",
//                     Some(&Direction::Down) => " v |",
//                     None => " * |",
//                 };
//                 print!("{}", to_print);
//             }

//             println!("");
//         }
//     }
// }

// impl Consumer for FHPRulesTestConsumer {

//     type Cell = FHP;

//     fn consume<G: Grid>(&mut self, grid: &mut G) {
//         assert_eq!(grid.cells().len(), 9);

//         self.pretty_print(grid);

//         // We are testing previous state.
//         let first: FHP = to_cell(&grid.cells()[0]);

//         match first.stage {
//             Stage::Collision => self.test_transport(grid),
//             Stage::Transport => self.test_collision(grid),
//         }
//     }
// }


// #[test]
// fn test_rules() {
//     // initial      collision    transport
//     // | * > < |    | * ^ v |    | * ^ * |
//     // | < * * | => | > * * | => | * > v |
//     // | > * * |    | > * * |    | * > * |
//     // 3x3 grid with 4 particles. There should be
//     // one rebound, head-on collision and simple move.

//     let left_particle = [false, true, false, false];
//     let right_particle = [false, false, true, false];

//     let cells = vec![
//         FHP { stage: Stage::Collision, particles: right_particle, coord: (1, 0) },
//         FHP { stage: Stage::Collision, particles: left_particle, coord: (2, 0) },
//         FHP { stage: Stage::Collision, particles: left_particle, coord: (0, 1) },
//         FHP { stage: Stage::Collision, particles: right_particle, coord: (0, 2) },
//     ];

//     let nhood = VonNeumannNhood::new();
//     let mut grid: SquareGrid<FHP, _> = SquareGrid::new(3, 3, nhood);
//     grid.set_cells(cells);

//     let right_particles_count =
//         grid.cells()
//             .iter()
//             .map(|c| to_cell(c))
//             .filter(|c: &FHP| c.particle(&Direction::Right) == true)
//             .count();
//     assert_eq!(right_particles_count, 2);

//     let consumer = FHPRulesTestConsumer::new();
//     let mut engine = Sequential::new(grid, consumer);
//     engine.run_times(2);
// }