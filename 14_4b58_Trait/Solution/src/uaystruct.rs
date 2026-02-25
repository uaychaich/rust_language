use crate::uaytrait::{ShapeTrait, TraitA, animals, live, plants};
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
/////////////////////////////////////////////////////////
pub struct Dog {
    pub name: String,
    pub number_of_legs: u8,
}
pub struct Rose {
    pub name: String,
    pub height: f64,
}

impl live for Dog {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl animals for Dog {
    fn get_number_of_legs(&self) -> u8 {
        self.number_of_legs
    }
}

impl live for Rose {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl plants for Rose {
    fn get_height(&self) -> f64 {
        self.height
    }
}
/////////////////////////////////////////////////////////
pub struct StructA;
impl TraitA for StructA {
    type Item1 = i32;
    type Item2 = String;
    fn method1(&self) -> Self::Item1 {
        42
    }
    fn method2(&self) -> Self::Item2 {
        "Hello from StructA".into()
    }
}
/////////////////////////////////////////////////////////