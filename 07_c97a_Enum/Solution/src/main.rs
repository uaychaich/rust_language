mod uayenum;
fn main() {
    let a1:uayenum::Fruit = uayenum::Fruit::Apple;
    //println!("a1={}",a1);
    match a1 {
        uayenum::Fruit::Apple => {println!("a1 is Apple")},
        uayenum::Fruit::Mango => {println!("a1 is Mango")},
        uayenum::Fruit::Orange => {println!("a1 is Orange")}
    }
    ///////////////////////////////////////////////////////////////////
    
    let a2:uayenum::TempDegree = uayenum::TempDegree::Celsius(37);
    match a2 {
        uayenum::TempDegree::None => {println!("No Data")},
        uayenum::TempDegree::Celsius(i) if i >= 40 => {println!("a2 is {} celsius, so hot",i)},
        uayenum::TempDegree::Celsius(i) if i >35 && i<40 => {println!("a2 is {} celsius, normal",i)},
        uayenum::TempDegree::Celsius(i) if i <=35 => {println!("a2 is {} celsius, so cold",i)},
        uayenum::TempDegree::Celsius(i) => {println!("a2 is {} celsius",i)},
        uayenum::TempDegree::Fahrenheit(i) => {println!("a2 is {} fahrenheit",i)},
    }

    
}
