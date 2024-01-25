use std::{fmt, f64};
use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn magnitude(&self) -> f64 {
        let res: f64 = (self.x.pow(2) + self.y.pow(2)) as f64;
        res.sqrt()
    }

    pub fn dist(&self, p2: Point) -> f64 {
        let res: f64 = ((p2.x - self.x).pow(2) + (p2.y - self.y).pow(2)) as f64;
        res.sqrt()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
        .field("x", &self.x)
        .field("y", &self.y)
        .finish()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon {
            points: Vec::<Point>::new(),
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn left_most_point(&self) -> Option<&Point> {
        self.points.iter().min_by_key(|p| p.x)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(*point);
            last_point = *point;
        }
        result += last_point.dist(self.points[0]);
        result
    }

}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Circle {
        Circle {
            center,
            radius
        }
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.radius)
    }

    pub fn dist(&self, other: &Self) -> f64 {
        self.center.dist(other.center)
    }
}


pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}