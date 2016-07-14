use std::marker::PhantomData;

use traits::Nhood;
use traits::Coord;


pub struct HexagonalNhood<C: Coord> {
    phantom: PhantomData<C>,
}


impl<C: Coord> HexagonalNhood<C> {

    pub fn new() -> Self { HexagonalNhood { phantom: PhantomData } }
}

impl<C: Coord> Nhood for HexagonalNhood<C> {
    type Coord = C;

    //  0 1
    // 2   3
    //  4 5
    fn neighbors(&self, coord: &Self::Coord) -> Vec<Self::Coord> {

        let x = coord.x();
        let y = coord.y();

        let neighbors_coords = vec![
                C::from_2d(x - 1, y - 1),
                C::from_2d(x, y - 1),

                C::from_2d(x - 1, y),
                C::from_2d(x + 1, y),

                C::from_2d(x - 1, y + 1),
                C::from_2d(x, y + 1),
        ];

        neighbors_coords
    }

    fn neighbors_count(&self) -> usize { 6 }
}


#[cfg(test)]
mod tests {

    use traits::Nhood;
    use super::HexagonalNhood;

    #[test]
    fn test_hexagonal_nhood() {
        let nhood = HexagonalNhood::new();

        let center = (1, 1);

        let neighbors = nhood.neighbors(&center);
        assert_eq!(neighbors.len(), nhood.neighbors_count());

        assert_eq!(neighbors[0], (0, 0));
        assert_eq!(neighbors[1], (1, 0));
        assert_eq!(neighbors[2], (0, 1));
        assert_eq!(neighbors[3], (2, 1));
        assert_eq!(neighbors[4], (0, 2));
        assert_eq!(neighbors[5], (1, 2));
    }
}