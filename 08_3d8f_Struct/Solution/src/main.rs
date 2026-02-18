mod uaystruct;
fn main() {
    let a1:uaystruct::Rectangle = uaystruct::Rectangle{width:23.5,height:33.5};
    println!("a1.width={} | a1.height={}",a1.width, a1.height);

    let a2:uaystruct::Rectangle = uaystruct::Rectangle{width:35.7,height:22.5};
    println!("a2.width={} | a2.height={}",a2.width, a2.height);
}
