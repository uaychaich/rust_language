fn main() {
    let a1:i32 = 5;
    let a2:&i32 = &a1;
    println!("a1={} | &a1={:p} | a2={:p} | *a2={}",a1, &a1, a2, *a2);
}
