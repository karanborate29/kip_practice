fn main() {
    println!("Hello, world!");
    println!("");
    fibo();
}

pub fn fibo(){

    let mut i =0;
    let mut a=0;
    let mut b=1;
    let mut c;
    //let mut x=100;

    while i<10 {

        if i>1 {
            c=b;//1//1//2//3
            b=b+a;//1//2//3//5
            a=c;//1//1//2//3

            println!("{}",b);
            i=i+1;

        }else {
            println!("{}",i);
            i=i+1;
        }
    }
}