use std::iter;

use utils::v2::V2;

#[derive(Copy, Clone)]
pub struct Rectangle {
    top_left: V2,
    bottom_right: V2,
}

impl Rectangle {
    pub fn new(top_left: V2, bottom_right: V2) -> Rectangle {
        assert!(top_left.0 <= bottom_right.0);
        assert!(top_left.1 <= bottom_right.1);
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    fn contains(&self, V2(y, x): V2) -> bool {
        let V2(y1, x1) = self.top_left;
        let V2(y2, x2) = self.bottom_right;
        (y1..y2).contains(&y) && (x1..x2).contains(&x)
    }

    // iter_nearest iterates over the points in the rectangle such that the points nearest the top
    // left are reached first. If you picture a 3x3 square, this is the order:
    //
    // 124
    // 357
    // 689
    pub fn iter_nearest(self) -> impl Iterator<Item = V2> {
        let start = self.top_left;
        let end = self.bottom_right - V2(1, 1);
        (0..=start.taxicab_distance(end)).flat_map(move |dist| {
            iter::successors(Some(V2(dist, 0)), |&pos| Some(pos + V2(-1, 1)))
                .map(move |pos| pos + start)
                .skip_while(move |pos| !self.contains(*pos))
                .take_while(move |pos| self.contains(*pos))
        })
    }
}

impl IntoIterator for Rectangle {
    type Item = V2;
    type IntoIter = RectangleIter;
    fn into_iter(self) -> Self::IntoIter {
        RectangleIter::new(self)
    }
}

impl RectangleIter {
    fn new(rect: Rectangle) -> RectangleIter {
        RectangleIter {
            rect,
            pos: rect.top_left - V2(0, 1),
        }
    }
}

pub struct RectangleIter {
    rect: Rectangle,
    pos: V2,
}

impl Iterator for RectangleIter {
    type Item = V2;

    fn next(&mut self) -> Option<Self::Item> {
        [
            self.pos + V2(0, 1),
            V2(self.pos.0 + 1, self.rect.top_left.1),
        ]
        .into_iter()
        .find(|&p| self.rect.contains(p))
        .map(|pos| {
            self.pos = pos;
            pos
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_iter_test() {
        let rect = Rectangle::new(V2(0, 0), V2(2, 2));
        let actual: Vec<_> = rect.into_iter().collect();
        let expected = vec![V2(0, 0), V2(0, 1), V2(1, 0), V2(1, 1)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn iter_nearest_test() {
        let rect = Rectangle::new(V2(0, 0), V2(3, 3));
        let actual: Vec<_> = rect.iter_nearest().collect();
        let expected = vec![
            V2(0, 0),
            V2(1, 0),
            V2(0, 1),
            V2(2, 0),
            V2(1, 1),
            V2(0, 2),
            V2(2, 1),
            V2(1, 2),
            V2(2, 2),
        ];
        assert_eq!(actual, expected);
    }
}
