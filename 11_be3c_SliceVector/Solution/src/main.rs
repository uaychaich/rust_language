fn main() {
    let mut a1:[i32;10] = [10,20,30,40,50,60,70,80,90,100];
    a1[1] = 700;
    println!("a1={:?}",a1);

    let sl1 = &mut a1[1..5];
    sl1[1] = 700;
    println!("sl1={:?}",sl1);

    let sl2:&mut [i32] = &mut [10,20,30];
    sl2[1] = 700;
    println!("sl2={:?}",sl2);
    
    let mut v1:Vec<i32> = vec![10,20,30,40,50,60,70,80,90,100];
    let mut v2:Vec<i32> = vec![110,120,130];
    v1.push(105);
    v1.append(&mut v2);
    v1[1]=700;
    v1.remove(2);
    println!("v1={:?}",v1);
}
