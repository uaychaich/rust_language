pub trait ShapeTrait {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn typename(&self) -> &'static str;
}
//////////////////////////////////////////////////////////
pub trait live {fn get_name(&self) -> String;}
pub trait animals: live {fn get_number_of_legs(&self) -> u8;}
pub trait plants: live {fn get_height(&self) -> f64;}
//////////////////////////////////////////////////////////
pub trait TraitA{
    type Item1;
    type Item2;
    fn method1(&self) -> Self::Item1;
    fn method2(&self) -> Self::Item2;
}