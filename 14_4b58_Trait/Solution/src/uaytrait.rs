pub trait ShapeTrait {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn typename(&self) -> &'static str;
}