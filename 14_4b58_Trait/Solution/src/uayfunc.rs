use uaytrait::ShapeTrait;

pub fn print_shape_info1(shape: &dyn ShapeTrait) {
    println!("Area1: {}", shape.area());
    println!("Perimeter1: {}", shape.perimeter());
    println!("Type1: {}", shape.typename());
}

pub fn print_shape_info2<T: ShapeTrait>(shape: &T) {
    println!("Area2: {}", shape.area());
    println!("Perimeter2: {}", shape.perimeter());
    println!("Type2: {}", shape.typename());
}

pub fn print_shape_info3<T>(shape: &T)
where
    T: ShapeTrait,
{
    println!("Area3: {}", shape.area());
    println!("Perimeter3: {}", shape.perimeter());
    println!("Type3: {}", shape.typename());
}