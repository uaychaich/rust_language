use MixCrate::lib_function;
use MixCrate::mod1::mod1filename_function;
use MixCrate::mod1::mod1sub1::mod1sub1filename_function;
use MixCrate::mod2::mod2filename_function;
use MixCrate::mod3::mod3inline_function;
use MixCrate::mod3::mod3sub1::mod3sub1inline_function;
use MixCrate::mod4::mod4inline_function;

fn main() {
    println!("Hello from the MixCrate binary!");
    let message = lib_function();
    println!("{}", message);
    let message = mod1filename_function();
    println!("{}", message);
    let message = mod1sub1filename_function();
    println!("{}", message);
    let message = mod2filename_function();
    println!("{}", message);
    let message = mod3inline_function();
    println!("{}", message);
    let message = mod3sub1inline_function();
    println!("{}", message);
    let message = mod4inline_function();
    println!("{}", message);
}
