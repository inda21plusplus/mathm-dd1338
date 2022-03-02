use crate::v;

use super::{polygon::Contains, Polygon, PolygonMethods};

#[test]
fn point_in_polygon_kattis() {
    // From: https://kth.kattis.com/problems/pointinpolygon
    let polygon = Polygon::<i32>::new(vec![v!(0, 0), v!(10, 0), v!(0, 10)]);
    assert_eq!(Contains::Inside, polygon.contains(v!(4, 5)));
    assert_eq!(Contains::OnBorder, polygon.contains(v!(5, 5)));
    assert_eq!(Contains::Outside, polygon.contains(v!(6, 5)));

    let polygon = Polygon::<i32>::new(vec![
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
    let polygon = Polygon::<i32>::new(vec![
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
    let polygon = Polygon::<i32>::new(vec![
        v![1, 2],
        v![1, 0],
        v![-2, 0],
        v![-2, 1],
        v![-1, 1],
        v![-1, 2],
    ]);
    assert_eq!(Contains::Inside, polygon.contains(v![0, 1]));
}

#[test]
fn polygon_is_convex() {
    let polygon = Polygon::<i32>::new(vec![v![1, 1], v![1, 2], v![0, 2], v![-1, 0], v![0, -1]]);
    assert!(polygon.is_convex());
    assert!(polygon.to_convex().is_ok());

    let polygon = Polygon::<i32>::new(vec![v![6, 1]]);
    assert!(polygon.is_convex());
    assert!(polygon.to_convex().is_ok());

    let polygon = Polygon::<i32>::new(vec![v![6, 1], v![12, 1]]);
    assert!(polygon.is_convex());
    assert!(polygon.to_convex().is_ok());

    let polygon = Polygon::<i32>::new(vec![v![1, 1], v![1, 2], v![1, 3], v![-1, 0], v![0, -1]]);
    assert!(!polygon.is_convex());
    assert!(polygon.to_convex().is_err());

    let polygon = Polygon::<i32>::new(vec![v![1, 0], v![0, 0], v![0, 1], v![-1, 0], v![0, -1]]);
    assert!(!polygon.is_convex());
    assert!(polygon.to_convex().is_err());
}

#[test]
fn polygon_equal() {
    let ps = [
        Polygon::<i32>::new(vec![v![0, 0], v![1, 1], v![-1, 1]]),
        Polygon::<i32>::new(vec![v![1, 1], v![-1, 1], v![0, 0]]),
        Polygon::<i32>::new(vec![v![-1, 1], v![0, 0], v![1, 1]]),
        Polygon::<i32>::new(vec![v![-1, 1], v![1, 1], v![0, 0]]),
        Polygon::<i32>::new(vec![v![0, 0], v![-1, 1], v![1, 1]]),
        Polygon::<i32>::new(vec![v![1, 1], v![0, 0], v![-1, 1]]),
    ];
    for a in &ps {
        for b in &ps {
            assert_eq!(a, b);
        }
    }
}

#[test]
fn convex_hull_kattis() {
    // From: https://open.kattis.com/problems/convexhull
    let polygon = Polygon::new(vec![v![0, 0], v![10, 0], v![0, 10]]);
    assert_eq!(
        Polygon::new(vec![v![0, 0], v![10, 0], v![0, 10]]),
        polygon.convex_hull()
    );

    let polygon = Polygon::<i32>::new(vec![
        v![41, -6],
        v![-24, -74],
        v![-51, -6],
        v![73, 17],
        v![-30, -34],
    ]);
    assert_eq!(
        Polygon::new(vec![v![-24, -74], v![73, 17], v![-51, -6]]),
        polygon.convex_hull()
    );

    let polygon = Polygon::<i32>::new(vec![v![50, 50], v![50, 50]]);
    assert_eq!(Polygon::new(vec![v![50, 50]]), polygon.convex_hull());
}

#[test]
fn convex_hull_edge_cases() {
    let polygon = Polygon::new(vec![v![0, 0], v![0, -1], v![0, 1], v![1, 0]]);
    assert_eq!(
        Polygon::new(vec![v![0, -1], v![0, 1], v![1, 0]]),
        polygon.convex_hull()
    );

    let polygon = Polygon::new(vec![
        v![0, 0],
        v![2, 0],
        v![1, 0],
        v![1, 1],
        v![0, 1],
        v![2, 1],
        v![30, 0],
        v![18, 0],
    ]);
    assert_eq!(
        Polygon::new(vec![v![0, 0], v![30, 0], v![2, 1], v![0, 1]]),
        polygon.convex_hull()
    );
}

#[test]
fn convex_hull_generated() {
    use rand::RngCore;

    let mut r = rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
    let mut c = || r.next_u32() as i32 as i64 / 2;
    for _ in 0..100 {
        let polygon = Polygon::<i64>::new((0..50).map(|_| v![c(), c()]).collect());
        let hull = polygon.clone().convex_hull();
        assert!(
            hull.unconvexify().is_convex(),
            "{:?} -> {:?}",
            polygon,
            hull
        );
        for &p in polygon.points() {
            assert!(hull.unconvexify().contains(p) != Contains::Outside);
        }
    }
}
