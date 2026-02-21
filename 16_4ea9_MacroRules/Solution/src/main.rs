mod uaymacro;
fn main() {
    uaymacro::uay1!();
    //////////////////////////////////////////////
    uaymacro::uay2!();
    uaymacro::uay2!{};
    uaymacro::uay2![];
    let i = vec!(1,2,3);
    println!{"i = {:?}", i};
    //////////////////////////////////////////////
    uaymacro::uay3!("Hello, world!");
    uaymacro::uay3!(42);
    let x = 3.14;
    uaymacro::uay3!(x+79.25);
    //////////////////////////////////////////////
    uaymacro::uay4!({
        println!("uay4! macro called with a block");
        println!("uay4! Kiki");
    });
    //////////////////////////////////////////////
    uaymacro::uay5!(uayvar, 100);
    println!("uayvar = {}", uayvar);
    //////////////////////////////////////////////
    uaymacro::uay6! {
        fn uay_function() {
            println!("uay6! macro called with an item");
        }
    }
    uay_function();
    //////////////////////////////////////////////
    uaymacro::uay7!(uayvar2, f64);
    uayvar2 = 3.14159;
    println!("uayvar2 = {}", uayvar2);
    //////////////////////////////////////////////
    
}
