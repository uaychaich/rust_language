fn main() {
    let x = 20;
    println!("The x is {x}");
    // x = 30;
    // println!("The x is {x}");

    let mut y = 200;
    println!("The y is {y}");
    y = 300;
    println!("The y is {y}");

    const Z:i32 = 2000;
    println!("The Z is {Z}");
    // z = 300;
    // println!("The x is {z}");

    let (i1, i2, i3, i4, i5):(i8,i16,i32,i64,i128) = (-5,-5,-5,-5,-5);
    let (u1, u2, u3, u4, u5):(u8,u16,u32,u64,u128) = (5,5,5,5,5);
    let (f1,f2):(f32,f64) = (5.23,7.56);
    let (b1,b2):(bool,bool)=(true,false);
    let c1:char = 'z';

    let t1:(i32,f32,bool)=(5,5.868,true);
    println!("{} {} {}",t1.0, t1.1, t1.2);
    let a1:[i32;3] = [1,2,3];
    println!("{} {} {}",a1[0],a1[1],a1[2]);
    let s1:&str ="Uaychai Naja";
    println!("{s1}");

    type Age = u8;
    let my_age:Age = 30;
    println!("My age is {my_age}");
}
