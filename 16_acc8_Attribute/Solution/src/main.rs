#[cold]
#[inline(always)]
#[deprecated(since = "0.0.5", note = "Use the `add_one` function instead")]
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let result = add_one(5);
    println!("Result: {}", result);
}
