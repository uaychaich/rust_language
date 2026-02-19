mod uaystruct;
mod uaytrait;
mod uayfunc;
fn main() {
    let circle = uaystruct::Circle { radius: 5.0 };
    let rectangle = uaystruct::Rectangle {
        width: 4.0,
        height: 6.0,
    };

    println!("Using print_shape_info1:");
    uayfunc::print_shape_info1(&circle);
    uayfunc::print_shape_info1(&rectangle);

    println!("\nUsing print_shape_info2:");
    uayfunc::print_shape_info2(&circle);
    uayfunc::print_shape_info2(&rectangle);

    println!("\nUsing print_shape_info3:");
    uayfunc::print_shape_info3(&circle);
    uayfunc::print_shape_info3(&rectangle);
}
