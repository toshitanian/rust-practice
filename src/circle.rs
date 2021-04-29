
use std::f64::consts::PI;
use crate::shape::Shape;

pub struct Circle {
    pub name: String,
    radius: i32,
}

impl Circle {
    pub fn new(name: String, radius: i32) -> Self {
        return Circle{name: name, radius: radius}
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        let f_radius: f64 = From::from(self.radius);
        return PI * f_radius.powf(2.0);
    }
}