enum ColorCode {
    rgb(u8, u8, u8),
    cmyk(u8, u8, u8, u8),
    hsl(u16, u8, u8),
}
struct Point {
    x: f64,
    y: f64,
}
fn main() {
    let a1 = [1, 2, 3, 4, 5, 6, 7];
    let a2 = (25.7, "hello");
    let a3 = ColorCode::rgb(255, 0, 0);
    let a4 = Point { x: 3.5, y: 4.2 };
    let a5 = 72;
    let a6 = &a5;
    ///////////////////////////////////////////////////////////////
    match a1 {
        [1, _, .., 1..5, i, j @ 1..10, k] if k>3 => println!("Matched array! => i: {}, j: {}, k: {}", i, j, k),
        _ => println!("Did not match array."),
    }   
    match a2 {
        (x @ 20.0..=30.0, y) if y.chars().next().unwrap() == 'h' => println!("Matched tuple! => x: {}, y: {}", x, y),
        _ => println!("Did not match tuple."),
    }
    match a3 {
        ColorCode::rgb(r @ 0..=255, g@ 0..=255, b @ 0..=255) if r > 200 && g == 0 && b == 0 => println!("Matched RGB color! => r: {}, g: {}, b: {}", r, g, b),
        _ => println!("Did not match RGB color."),
    }
    match a4 {
        Point { x: x @ 0.0..=5.0, y: y @ 0.0..=5.0 } => println!("Matched Point! => x: {}, y: {}", x, y),
        _ => println!("Did not match Point."),
    }
    match a6 {
        &x @ 0..=100 if x % 2 == 0 => println!("Matched reference to even number! => x: {}", x),
        _ => println!("Did not match reference."),
    }
    match *a6 {
        x @ 0..=100 if x % 2 == 0 => println!("Matched dereferenced even number! => x: {}", x),
        _ => println!("Did not match dereferenced value."),
    }
    ////////////////////////////////////////////////////////////////////////////////////
    if let [1, _, .., 1..5, i, j @ 1..10, k] = a1 {
        if k > 3 {
            println!("Matched array with if let! => i: {}, j: {}, k: {}", i, j, k);
        } else {
            println!("Array matched but condition not met.");
        }
    } else {
        println!("Did not match array with if let.");
    }
    ////////////////////////////////////////////////////////////////////////////////////
    let mut my_option: Option<i32> = Some(1);
    while let Some(i) = my_option {
        if i>3 {
            println!("End of loop with value: {}", i);
            my_option = None; 
        }else{
            println!("Matched Some! => i: {}", i);
            my_option = Some(i + 1); 
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////
    let uaystring = "45";
    let Ok(i) =uaystring.parse::<i32>() else {
        println!("Did not match Ok variant.");
        return;
    };
    println!("Parsed integer: {}", i);

}
