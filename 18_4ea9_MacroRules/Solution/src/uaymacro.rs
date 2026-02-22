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

#[macro_export]
macro_rules! uay8 {
    ($x:ident,$y:path) => {let $x:$y;};
}

#[macro_export]
macro_rules! uay9 {
    ($x:expr,$y:pat) => {match $x {
        $y => println!("uay9! macro called with pattern: {}", stringify!($y)),
        _ => println!("uay9! macro called with non-matching pattern: {}", stringify!($y)),
    }
    };
}

#[macro_export]
macro_rules! uay10 {
    ($x:stmt) => {$x;};
}

#[macro_export]
macro_rules! uay11 {
    ($x:tt) => {println!("uay11! macro called with token tree: {}", stringify!($x))};
}

#[macro_export]
macro_rules! uay12 {
    ($($x:tt)*) => {println!("uay12! macro called with token tree: {}", stringify!($($x)*))};
}

#[macro_export]
macro_rules! uay13 {
    ($x:meta) => {#[$x] fn uay_meta_function() {
        println!("uay13! macro called with meta item: {}", stringify!($x));
    }};
}

pub(crate) use uay1;
pub(crate) use uay2;
pub(crate) use uay3;
pub(crate) use uay4;
pub(crate) use uay5;
pub(crate) use uay6;
pub(crate) use uay7;
pub(crate) use uay8;
pub(crate) use uay9;
pub(crate) use uay10;
pub(crate) use uay11;
pub(crate) use uay12;
pub(crate) use uay13;