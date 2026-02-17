fn uay_func1(){
    println!("Hello UayFunc1");
}

pub fn uay_func2(){
    uay_func1();
}

pub fn uay_func3()->i32{
    return 35;
}

pub fn uay_func4()->(i32,i32){
    return (35,37);
}

pub fn uay_func5()->(i32,i32){
    (35,37)
}

pub fn uay_func6(x1:i32, x2:i32)->i32{
    x1+x2
}

pub fn uay_func7(x1:i32,x2:Option<i32>)->i32{
    x1+x2.unwrap_or(11)
}

pub fn uay_func9(x1:i32,x2:i32,x3:impl Fn(i32,i32)->i32)->i32{
    x3(x1,x2)
}

pub fn uay_func101(x1:i32, x2:impl Fn(i32)->()){
    x2(x1);
    x2(x1);
}

pub fn uay_func102(x1:i32, mut x2:impl FnMut(i32)->()){
    x2(x1);
    x2(x1);
}

pub fn uay_func103(x1:i32, x2:impl FnOnce(i32)->()){
    x2(x1);
    //x2(x1);
}

pub fn uay_func11(x1:i32)->impl Fn(i32,i32)->i32{
    move |a1,a2| a1+a2+x1
}