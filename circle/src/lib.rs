use std::f64::consts::PI;


#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, b: Point) -> f64 {
        ((b.0 - &self.0).powi(2) + (b.1 - &self.1).powi(2)).sqrt()
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        &self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        &self.radius.powi(2) * PI
    }
    pub fn intersect(&self, c: Circle) -> bool {
        let d = self.center.distance(c.center);
        if d > self.radius + c.radius {
            return false;
        } else {
            return true;
        }
    }
}