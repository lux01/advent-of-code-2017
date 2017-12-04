pub struct SpiralIterator {
    number: i32,
}

impl SpiralIterator {
    pub fn new() -> SpiralIterator {
        SpiralIterator { number: 0 }
    }
}


impl Iterator for SpiralIterator {
    type Item = ((i32, i32), i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.number += 1;
        let n = (((self.number as f64).sqrt() - 1.0) / 2.0).ceil() as i32;

        let top_right = 4 * n * n - 2 * n + 1;
        let top_left = 4 * n * n + 1;
        let bottom_left = 4 * n * n + 2 * n + 1;
        let bottom_right = 4 * n * n + 4 * n + 1;

        let (x, y) = if self.number < top_right {
            (n, n + self.number - top_right)
        } else if top_right <= self.number && self.number < top_left {
            (n + top_right - self.number, n)
        } else if top_left <= self.number && self.number < bottom_left {
            (-n, n + top_left - self.number)
        } else if bottom_left <= self.number && self.number < bottom_right {
            (-n + (self.number - bottom_left), -n)
        } else {
            (n + self.number - bottom_right, -n)
        };

        Some(((x, y), self.number))
    }
}

#[test]
fn test() {
    let mut iter = SpiralIterator::new();

    assert_eq!(Some(((0, 0), 1)), iter.next());
    assert_eq!(Some(((1, 0), 2)), iter.next());
    assert_eq!(Some(((1, 1), 3)), iter.next());
    assert_eq!(Some(((0, 1), 4)), iter.next());
    assert_eq!(Some(((-1, 1), 5)), iter.next());
    assert_eq!(Some(((-1, 0), 6)), iter.next());
    assert_eq!(Some(((-1, -1), 7)), iter.next());
    assert_eq!(Some(((0, -1), 8)), iter.next());
    assert_eq!(Some(((1, -1), 9)), iter.next());
    assert_eq!(Some(((2, -1), 10)), iter.next());
    assert_eq!(Some(((2, 0), 11)), iter.next());
    assert_eq!(Some(((2, 1), 12)), iter.next());
    assert_eq!(Some(((2, 2), 13)), iter.next());
    assert_eq!(Some(((1, 2), 14)), iter.next());
    assert_eq!(Some(((0, 2), 15)), iter.next());
}
