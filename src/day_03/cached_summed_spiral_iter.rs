use std::collections::HashMap;
use super::spiral_iter::SpiralIterator;


pub struct CachedSummedSpiralIterator {
    history: HashMap<(i32, i32), i32>,
    iter: Box<Iterator<Item = ((i32, i32), i32)>>,
}

impl CachedSummedSpiralIterator {
    pub fn new() -> CachedSummedSpiralIterator {
        let mut history = HashMap::new();
        history.insert((0, 0), 1);
        let iter = Box::new(SpiralIterator::new().skip(1));

        CachedSummedSpiralIterator { history, iter }
    }
}

impl Iterator for CachedSummedSpiralIterator {
    type Item = ((i32, i32), i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((x, y), _)) = self.iter.next() {
            let sum_of_neighbours = [
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
            ].into_iter()
                .map(|pos| {
                    let val = self.history.get(&pos).map(|&n| n).unwrap_or(0);
                    val
                })
                .sum::<i32>();
            self.history.insert((x, y), sum_of_neighbours);

            Some(((x, y), sum_of_neighbours))
        } else {
            None
        }
    }
}

#[test]
fn test_cached_spiral_iter() {
    let mut iter = CachedSummedSpiralIterator::new();

    assert_eq!(Some(((1, 0), 1)), iter.next());
    assert_eq!(Some(((1, 1), 2)), iter.next());
    assert_eq!(Some(((0, 1), 4)), iter.next());
    assert_eq!(Some(((-1, 1), 5)), iter.next());
    assert_eq!(Some(((-1, 0), 10)), iter.next());
    assert_eq!(Some(((-1, -1), 11)), iter.next());
    assert_eq!(Some(((0, -1), 23)), iter.next());
    assert_eq!(Some(((1, -1), 25)), iter.next());
    assert_eq!(Some(((2, -1), 26)), iter.next());
    assert_eq!(Some(((2, 0), 54)), iter.next());
    assert_eq!(Some(((2, 1), 57)), iter.next());
    assert_eq!(Some(((2, 2), 59)), iter.next());
    assert_eq!(Some(((1, 2), 122)), iter.next());
}
