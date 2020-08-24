//Assignment Four
fn main() {
    println!("Assignment Four\n");

    duplicate();        //Q1
    rotation();         //Q2
    palindrome();       //Q3

}

pub fn duplicate() {
    let input = String::from("hello world");
    let mut strr = String::new();

    for i in input.chars() {
        let mut count: i32 = 0;

        for j in input.chars() {
            if i == j {
                count = count + 1;
            }
        }
        if count > 1 {
            if strr.contains(i) {
                continue
            } else {
                strr.push(i);
            }
        }
    }
    for n in strr.chars() {
        println!("{}", n);
    }
}

fn rotation() {
    let input1:String = String::from("ZYXW");
    let input2:String = String::from("WXYZ");
    let length = input2.len()-1;
    rotation_recursive(input1, input2, 0 as usize, length as usize,  length as usize);
}
fn rotation_recursive(input1:String ,input2:String , start:usize, end:usize, length:usize ) {

    let startone: i32 = input1.as_bytes()[start] as i32;
    let endtwo: i32 = input2.as_bytes()[end] as i32;

    if length==0 {
        println!("\nCorrect\n");
    }else if startone == endtwo {
        rotation_recursive(input1, input2, (start+1) as usize, (end - 1) as usize, length-1);
    }else {
        println!("\nNot Corrrect\n");
    }
}

fn palindrome() {
    let input:String = String::from("MOA");
    let length = input.len()-1;
    palindrom_recursive(input, 0 as usize, length as usize,  length as usize);
}


fn palindrom_recursive(input:String , start:usize, end:usize, length:usize ) {
    let startone: i32 = input.as_bytes()[start] as i32;
    let endone: i32 = input.as_bytes()[end] as i32;

    if length==0 {
        println!("Palindrome\n");
    }else if startone == endone {
        println!("i {} s {} e {} l {}",input, start, end, length);
        palindrom_recursive(input, (start+1) as usize, (end - 1) as usize, length-1);
    }else {
        println!("Not Palindrome\n");
    }
}