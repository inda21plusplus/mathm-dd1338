use std::ops;

use super::{line::Side, LineSegment, Numeric, Vector};

pub struct Polygon<T: Numeric> {
    points: Vec<Vector<T, 2>>,
}

pub struct ConvexPolygon<T: Numeric> {
    points: Vec<Vector<T, 2>>,
}

impl<T: Numeric> Polygon<T> {
    pub fn new(points: Vec<Vector<T, 2>>) -> Self {
        Self { points }
    }
    pub fn is_convex(&self) -> bool {
        todo!()
    }
    pub fn convex(self) -> Result<ConvexPolygon<T>, Polygon<T>> {
        let is_convex = self.is_convex();
        let points = self.points;
        if is_convex {
            Ok(ConvexPolygon { points })
        } else {
            Err(Polygon { points })
        }
    }
    pub fn lines<'a>(&'a self) -> Lines<'a, T> {
        Lines {
            polygon: self,
            i: 0,
        }
    }

    /// Checks whether a point is inside, outside of, or on the border of the polygon.
    /// For integer vectors there will be no rounding errors.
    /// # Time complexity
    /// O(*n*) where n is the amount of points in the polygon.
    pub fn contains(&self, point: Vector<T, 2>) -> Contains {
        let mut inside = false;
        let x_line = LineSegment::new(point, point + Vector::from([T::ONE, T::ZERO]));
        let y_line = LineSegment::new(point, point + Vector::from([T::ZERO, T::ONE]));
        for segment in self.lines() {
            if segment.0 == point {
                return Contains::OnBorder;
            }
            let top = if segment.0[1] > segment.1[1] {
                segment.0
            } else {
                segment.1
            };
            let bot = segment.0 + segment.1 - top;
            match (x_line.side(top), x_line.side(bot)) {
                (Side::Left, Side::Right) | (Side::Right, Side::Left) => {
                    match LineSegment::new(top, bot).side(point) {
                        Side::Left => inside = !inside,
                        Side::On => return Contains::OnBorder,
                        Side::Right => {}
                    }
                }
                (Side::On, Side::On) => match (y_line.side(top), y_line.side(bot)) {
                    (Side::Left, Side::Right) | (Side::Right, Side::Left) => {
                        return Contains::OnBorder
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        let idx = |mut i: isize| {
            while i < 0 {
                i += self.len() as isize;
            }
            while i >= self.len() as isize {
                i -= self.len() as isize;
            }
            self[i as usize]
        };
        let mut i = 0;
        while i < self.len() as isize {
            if x_line.side(idx(i)) == Side::On && y_line.side(idx(i)) == Side::Left {
                let mut p = i - 1;
                while x_line.side(idx(p)) == Side::On {
                    p -= 1;
                }
                let mut n = i + 1;
                while x_line.side(idx(n)) == Side::On {
                    n += 1;
                }
                if x_line.side(idx(p)) != x_line.side(idx(n)) {
                    inside = !inside;
                }
                i = n;
            } else {
                i += 1;
            }
        }
        if inside {
            Contains::Inside
        } else {
            Contains::Outside
        }
    }
}

impl<T: Numeric> ops::Deref for Polygon<T> {
    type Target = Vec<Vector<T, 2>>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl<T: Numeric> ops::DerefMut for Polygon<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

impl<T: Numeric> ops::Index<usize> for ConvexPolygon<T> {
    type Output = Vector<T, 2>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Contains {
    Inside,
    OnBorder,
    Outside,
}

pub struct Lines<'a, T: Numeric> {
    polygon: &'a Polygon<T>,
    i: usize,
}

impl<'a, T: Numeric> Iterator for Lines<'a, T> {
    type Item = LineSegment<T, 2>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.polygon.points.len() - 1 {
            self.i += 1;
            Some(LineSegment::new(
                self.polygon.points[self.i - 1],
                self.polygon.points[0],
            ))
        } else if self.i < self.polygon.points.len() {
            self.i += 1;
            Some(LineSegment::new(
                self.polygon.points[self.i - 1],
                self.polygon.points[self.i],
            ))
        } else {
            None
        }
    }
}
