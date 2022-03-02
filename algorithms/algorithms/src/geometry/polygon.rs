use std::{cmp::Ordering, ops};

use super::{line::Side, Line, LineSegment, Numeric, Vector};

#[derive(Debug, Clone)]
pub struct Polygon<T: Numeric, const CONVEX: bool = false> {
    points: Vec<Vector<T, 2>>,
}

/// Methods defined for all types of polygons but which might have different implementations
/// depending on compile time constants.
///
/// In time complexities, *n* is the amount of points in the polygon.
pub trait PolygonMethods<T: Numeric> {
    /// Returns an iterator over the line segments of `self`
    fn lines<'a>(&'a self) -> Lines<'a, T>;

    /// Checks whether the polygon is convex. Polygons with zero, one, or two points are always
    /// considered convex. Polygons with three or more co-linear points are not considered convex.
    ///
    /// # Time complexity
    /// O(1) if `self` is known to be convex,
    /// O(*n*) otherwise
    fn is_convex(&self) -> bool;

    /// Returns smallest convex polygon containing all points in `self`.
    ///
    /// # Time complexity
    /// O(*n*) if `self` is known to be convex
    /// O(*n* log *n*) otherwise
    fn convex_hull(self) -> Polygon<T, true>;

    /// Checks whether a point is inside, outside of, or on the border of the polygon.
    /// For integer vectors there will be no rounding errors.
    ///
    /// # Time complexity
    /// O(*n*)
    fn contains(&self, point: Vector<T, 2>) -> Contains;
}

impl<T: Numeric, const CONVEX: bool> Polygon<T, CONVEX> {
    pub fn len(&self) -> usize {
        self.points.len()
    }
    pub fn points(&self) -> &[Vector<T, 2>] {
        &self.points
    }
    pub fn remove_duplicates(&mut self) {
        self.points.dedup();
    }
}

impl<T: Numeric> Polygon<T, true> {
    /// Returns `self` as a not neccessarily convex `Polygon`.
    pub fn unconvexify(&self) -> &Polygon<T, false> {
        // SAFETY: They're the same god dang thing
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Numeric> Polygon<T, false> {
    pub fn new(points: Vec<Vector<T, 2>>) -> Self {
        Self { points }
    }

    /// Returns `self` as a convex `Polygon` if `self` is convex. For getting the *convex hull* of
    /// `self`, see `convex_hull`. If you are absolutely certain that `self` is convex and don't
    /// want to verify that during run time, see `to_convex_unchecked`
    /// # Time complexity
    /// O(*n*) where n is the amount of points in `self`
    pub fn to_convex(self) -> Result<Polygon<T, true>, Polygon<T, false>> {
        let is_convex = self.is_convex();
        let points = self.points;
        if is_convex {
            Ok(Polygon { points })
        } else {
            Err(Polygon { points })
        }
    }

    /// Returns `self` as a convex `Polygon` even if `self` is not convex. If `self` is in fact not
    /// convex, methods on the result may return invalid results.
    pub fn to_convex_unchecked(self) -> Polygon<T, true> {
        Polygon::<T, true> {
            points: self.points,
        }
    }
}

impl<T: Numeric> PolygonMethods<T> for Polygon<T, true> {
    fn lines<'a>(&'a self) -> Lines<'a, T> {
        Lines {
            polygon: &self.unconvexify(),
            i: 0,
        }
    }

    fn is_convex(&self) -> bool {
        true
    }

    fn convex_hull(mut self) -> Polygon<T, true> {
        self.remove_duplicates();
        self
    }

    fn contains(&self, _point: Vector<T, 2>) -> Contains {
        todo!()
    }
}

impl<T: Numeric> PolygonMethods<T> for Polygon<T, false> {
    fn lines<'a>(&'a self) -> Lines<'a, T> {
        Lines {
            polygon: self,
            i: 0,
        }
    }

    fn is_convex(&self) -> bool {
        if self.len() < 3 {
            return true;
        }
        let side = LineSegment::new(self[0isize], self[2isize]).side(self[1isize]);
        if side == Side::On {
            return false;
        }
        for i in 0..self.len() as isize {
            if LineSegment::new(self[i - 1], self[i + 1]).side(self[i]) != side {
                return false;
            }
        }
        true
    }

    fn convex_hull(self) -> Polygon<T, true> {
        let mut ps = self.points;
        let cmp = |(_, a): &(usize, &Vector<T, 2>), (_, b): &(usize, &Vector<T, 2>)| {
            if a[0] < b[0] || a[0] == b[0] && a[1] < b[1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };

        let &leftmost = ps.iter().enumerate().min_by(cmp).unwrap().1;
        ps.retain(|&p| p != leftmost);

        ps.sort_by(|&a, &b| match Line::new(leftmost, b).side(a) {
            Side::Right => Ordering::Less,
            Side::On => {
                if a[0] < b[0] {
                    Ordering::Less
                } else if a[0] > b[0] {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            Side::Left => Ordering::Greater,
        });

        let mut hull = vec![leftmost];
        for &p in &ps {
            while hull.len() >= 2 {
                let line = Line::new(hull[hull.len() - 2], hull[hull.len() - 1]);
                if line.side(p) == Side::Left {
                    break;
                }
                hull.pop();
            }
            if hull.last() != Some(&p) {
                hull.push(p);
            }
        }

        if hull.len() >= 3
            && Line::new(hull[0], hull[hull.len() - 2]).side(hull[hull.len() - 1]) == Side::On
        {
            hull.pop();
        }

        Polygon::new(hull).to_convex_unchecked()
    }

    fn contains(&self, point: Vector<T, 2>) -> Contains {
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
        let mut i = 0;
        while i < self.len() as isize {
            if x_line.side(self[i]) == Side::On && y_line.side(self[i]) == Side::Left {
                let mut p = i - 1;
                while x_line.side(self[p]) == Side::On {
                    p -= 1;
                }
                let mut n = i + 1;
                while x_line.side(self[n]) == Side::On {
                    n += 1;
                }
                if x_line.side(self[p]) != x_line.side(self[n]) {
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

impl<T: Numeric, const CONVEX: bool> ops::Index<usize> for Polygon<T, CONVEX> {
    type Output = Vector<T, 2>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.points[i]
    }
}

impl<T: Numeric, const CONVEX: bool> ops::Index<isize> for Polygon<T, CONVEX> {
    type Output = Vector<T, 2>;

    fn index(&self, i: isize) -> &Self::Output {
        if i >= self.len() as isize {
            &self.points[i as usize - self.len()]
        } else if i < 0 {
            &self.points[(i + self.len() as isize) as usize]
        } else {
            &self.points[i as usize]
        }
    }
}

impl<T: Numeric> ops::IndexMut<usize> for Polygon<T, false> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.points[i]
    }
}

/// Checks whether two polygons are equal. Two polygons with the same points in the same order are
/// concidered equal. They can still be equal if they have different "starting" points and if their
/// points are given in opposide orders.
/// # Time complexity
/// O(*n* * *m*) where *m* is the amount of points considered equally minimum (in the first axis
/// and then the second axis). Which is in the worst case O(*n*^2)
impl<T: Numeric, const CONVEX_A: bool, const CONVEX_B: bool> PartialEq<Polygon<T, CONVEX_B>>
    for Polygon<T, CONVEX_A>
{
    fn eq(&self, other: &Polygon<T, CONVEX_B>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut a_min = vec![0];
        for i in 1..self.len() {
            let u = self[i];
            let v: Vector<T, 2> = self[a_min[0]];
            if (u[0], u[1]) < (v[0], v[1]) {
                a_min.clear();
                a_min.push(i);
            } else if u == v {
                a_min.push(i);
            }
        }
        let mut b_min = vec![0];
        for i in 1..other.len() {
            let u = other[i];
            let v: Vector<T, 2> = other[b_min[0]];
            if (u[0], u[1]) < (v[0], v[1]) {
                b_min.clear();
                b_min.push(i);
            } else if u == v {
                b_min.push(i);
            }
        }

        if a_min.len() != b_min.len() || a_min.len() == 0 {
            return false;
        }

        for offset in 0..a_min.len() {
            for (&a, &b) in a_min
                .iter()
                .zip(b_min.iter().skip(offset).chain(b_min.iter().take(offset)))
            {
                if self.points[a..]
                    .iter()
                    .chain(self.points[..a].iter())
                    .zip(other.points[b..].iter().chain(other.points[..b].iter()))
                    .all(|(u, v)| u == v)
                    || self.points[a..]
                        .iter()
                        .chain(self.points[..a].iter())
                        .zip(
                            other.points[b + 1..]
                                .iter()
                                .chain(other.points[..=b].iter())
                                .rev(),
                        )
                        .all(|(u, v)| u == v)
                {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Contains {
    Inside,
    OnBorder,
    Outside,
}

pub struct Lines<'a, T: Numeric> {
    polygon: &'a Polygon<T, false>,
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
