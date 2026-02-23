use LibraryCrate::lib_function;
use LibraryCrate::mod1::mod1filename_function;
use LibraryCrate::mod1::mod1sub1::mod1sub1filename_function;
use LibraryCrate::mod2::mod2filename_function;
use LibraryCrate::mod3::mod3inline_function;
use LibraryCrate::mod3::mod3sub1::mod3sub1inline_function;
use LibraryCrate::mod4::mod4inline_function;

fn main() {
    println!("Hello, world! from the BinaryCrate binary!");
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
