mod uayfunc;
fn main() {
    
    uayfunc::uay_func2();

    let a3 = uayfunc::uay_func3();
    println!("a3={}",a3);

    let (a41,a42)= uayfunc::uay_func4();
    println!("a41={} | a42={}", a41,a42);

    let (a51,a52)=uayfunc::uay_func5();
    println!("a51={} | a52={}", a51,a52);

    let a61 = uayfunc::uay_func6(77, 22);
    println!("a61={}",a61);

    let a71 = uayfunc::uay_func7(77, None);
    let a72 = uayfunc::uay_func7(77, Some(22));
    println!("a71={} | a72={}", a71, a72);

    let a81 = |x1:i32,x2:i32|->i32 {x1+x2};
    let a82 = |x1,x2| x1+x2;
    println!("a81={} | a82={}", a81(5,2), a82(5,2));

    let a91 = uayfunc::uay_func9(55, 33, |a,b| a+b);
    let a92 = uayfunc::uay_func9(55, 33, |a,b| a-b);
    println!("a91={} | a92={}", a91, a92);

    let (mut a101, mut a102, mut a103) = (33,33,33);
    //uayfunc::uay_func101(44, |z| a101+=z);
    uayfunc::uay_func102(44, |z| a102+=z);
    uayfunc::uay_func103(44, |z| a103+=z);
    println!("a101={} | a102={} | a103={}",a101, a102, a103);

    let fa11 = uayfunc::uay_func11(21);
    let a111 = fa11(11,22);
    let a112 = fa11(100,200);
    println!("a111={} | a112={}",a111, a112);

}
