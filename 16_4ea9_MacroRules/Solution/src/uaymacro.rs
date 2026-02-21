#[macro_export]
macro_rules! uay1 {
    () => {println!("uay1! macro called")};
}

#[macro_export]
macro_rules! uay2 {
    {} => [println!("uay2! macro called with empty braces");];
    [] => (println!("uay2! macro called with empty brackets"););
    () => {println!("uay2! macro called with empty parentheses");};
}

#[macro_export]
macro_rules! uay3 {
    ($x:expr) => {println!("uay3! macro called with argument: {}", $x)};
}

#[macro_export]
macro_rules! uay4 {
    ($x:block) => { println!("uay4! macro called with block: {}", stringify!($x));
                    $x};
}

#[macro_export]
macro_rules! uay5 {
    ($x:ident,$y:literal) => {let $x = $y;};
}

#[macro_export]
macro_rules! uay6 {
    ($x:item) => {$x};
}

#[macro_export]
macro_rules! uay7 {
    ($x:ident,$y:ty) => {let $x:$y;};
}

pub(crate) use uay1;
pub(crate) use uay2;
pub(crate) use uay3;
pub(crate) use uay4;
pub(crate) use uay5;
pub(crate) use uay6;
pub(crate) use uay7;