fn main() {
    let mut a1:i32 = 0;
    loop {
        a1+=1;
        if a1==3 {continue;}
        if a1==9 {break;}
        println!("a1={}",a1);
    }
    println!("///////////////////////////////////");
    ////////////////////////////////////////////////////
    let mut a21:i32 = 0;
    
    let a22 = loop {
        a21+=1;
        if a21==3 {continue;}
        if a21==9 {break a21;}
        println!("a21={}",a21);
    };
    println!("a22={}",a22);
    println!("///////////////////////////////////");
    ////////////////////////////////////////////////////
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            //break;
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
    println!("///////////////////////////////////");
    ////////////////////////////////////////////////////
    let mut a31:i32 = 0;
    let mut a32:i32 = 0;

    'outer:while a31 < 10 {
        a31+=1;a32=0;

        'inner: while a32< 5{
            a32+=1;
            if a31==2 && a32==2 {break 'outer;}
            println!("a31={} | a32={}",a31,a32);
        }
    }
    println!("///////////////////////////////////");
    ////////////////////////////////////////////////////
    for a41 in 1..10{
        println!("a41={}",a41)
    }
    println!("///////////////////////////////////");
    ////////////////////////////////////////////////////
    let a51:[i32;3] = [10,20,30];
    for a52 in a51.iter(){
        println!("a52={}",a52)
    }
    println!("///////////////////////////////////");
}
