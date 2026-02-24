extern crate ProceduralMacro;
use ProceduralMacro::uay_proc_macro1;
use ProceduralMacro::uay_proc_macro2;
use ProceduralMacro::uayprocmacro3;

#[derive(uayprocmacro3)]
struct UayPeople {
    name: String,
    age: u32,
}

#[ProceduralMacro::uay_proc_macro4("This is an attribute value")]
fn uaynaja() -> () {
    println!("This is a function from the main.rs file");
}

fn main() {
    let a1 = uay_proc_macro1!();
    println!("The a1 is: {}", a1);
    let a2= uay_proc_macro2!("Uaychai");
    println!("The a2 is: {}", a2);

    let people = UayPeople {
        name: "Uaychai".to_string(),
        age: 30,
    };
    println!("{}", people.showme());
    uaynaja();
}
