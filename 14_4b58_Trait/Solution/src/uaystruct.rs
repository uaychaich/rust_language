use crate::uaytrait::ShapeTrait;
use std::any::type_name;

pub struct Circle {
    pub radius: f64,
}
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl ShapeTrait for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    fn typename(&self) -> &'static str {
        type_name::<Circle>()
    }
}
impl ShapeTrait for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    fn typename(&self) -> &'static str {
        type_name::<Rectangle>()
    }
}