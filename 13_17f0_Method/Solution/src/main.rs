mod uaystruct;
fn main() {
    let rect1 = uaystruct::Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rect1 is {} square pixels.", rect1.area());
    println!("The perimeter of the rect1 is {} pixels.", rect1.perimeter());
    let rect2 = uaystruct::Rectangle {
        width: 10,
        height: 40,
    };
    println!("The area of the rect2 is {} square pixels.", rect2.area());
    println!("The perimeter of the rect2 is {} pixels.", rect2.perimeter());
}
