// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::{ops::Add, slice::Iter, f64::consts::PI};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    a: f64,
    b: f64
}

impl Point {
    fn new(a: i32, b: i32) -> Self {
        Self { a: a.into(), b: b.into() }
    }

    fn dist(&self, p2: Point) -> f64 {
        let a2 = (self.a - p2.a).powi(2);
        let b2 = (self.b - p2.b).powi(2);
        (a2 + b2).sqrt()
    }

    fn magnitude(&self) -> f64 {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self { a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }  

    fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            return None
        }
        let mut m: &Point = &self.points[0];
        for p in &self.points {
            if p.a < m.a {
                m = p;
            }
        }
        Some(m.clone())
    }

    fn iter(&self) -> Iter<Point>{
        self.points.iter()
    }
}

// impl Into<Shape> for Polygon {
//     fn into(self) -> Shape {
//         Shape::Polygon(self)
//     }
// }

pub struct Circle {
    center: Point,
    radius: u32
}

impl Circle {
    fn new(c: Point, r: u32) -> Self {
        Self { center: c, radius: r}
    }
}

// impl Into<Shape> for Circle {
//     fn into(self) -> Shape {
//         Shape::Circle(self)
//     }
// }

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => 2.0 * PI * c.radius as f64,
            Shape::Polygon(v) => {
                let mut dist = 0_f64;
                let l = v.points.len();
                for i in 0..l {
                    dist += v.points[i].dist(v.points[(i+1)%l]);
                }
                dist
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}