use crate::v;

use super::{polygon::Contains, Polygon, Vector};

#[test]
fn point_in_polygon_kattis() {
    // From: https://kth.kattis.com/problems/pointinpolygon
    let polygon = Polygon::new(vec![v!(0, 0), v!(10, 0), v!(0, 10)]);
    assert_eq!(Contains::Inside, polygon.contains(v!(4, 5)));
    assert_eq!(Contains::OnBorder, polygon.contains(v!(5, 5)));
    assert_eq!(Contains::Outside, polygon.contains(v!(6, 5)));

    let polygon = Polygon::new(vec![
        v!(41, -6),
        v!(-24, -74),
        v!(-51, -6),
        v!(73, 17),
        v!(-30, -34),
    ]);
    assert_eq!(Contains::Outside, polygon.contains(v!(-12, -26)));
    assert_eq!(Contains::Inside, polygon.contains(v!(38, -8)));
}

#[test]
fn point_in_polygon_through_point() {
    let polygon = Polygon::new(vec![
        v![2, 2],
        v![1, 0],
        v![2, -2],
        v![0, -2],
        v![-2, -2],
        v![-2, 0],
        v![-2, 2],
        v![0, 2],
    ]);
    assert_eq!(Contains::Inside, polygon.contains(v![0, 0]));
    assert_eq!(Contains::OnBorder, polygon.contains(v![-2, 0]));
    assert_eq!(Contains::OnBorder, polygon.contains(v![0, 2]));
    assert_eq!(Contains::OnBorder, polygon.contains(v![1, 2]));
    assert_eq!(Contains::OnBorder, polygon.contains(v![-2, 1]));
    assert_eq!(Contains::OnBorder, polygon.contains(v![-2, -2]));
    assert_eq!(Contains::Outside, polygon.contains(v![2, -1]));
    assert_eq!(Contains::Outside, polygon.contains(v![2, 0]));
    assert_eq!(Contains::Outside, polygon.contains(v![2, 1]));
}

#[test]
fn point_in_polygon_through_line() {
    let polygon = Polygon::new(vec![
        v![1, 2],
        v![1, 0],
        v![-2, 0],
        v![-2, 1],
        v![-1, 1],
        v![-1, 2],
    ]);
    assert_eq!(Contains::Inside, polygon.contains(v![0, 1]));
}
