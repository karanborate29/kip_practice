extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!\n");
    opration_on_complex();   //Q1
    opration_on_struct();   //Q2->1->2->3
}

struct DataInit {
    num1: num::Complex<i32>,
    num2: num::Complex<i32>,
}

fn opration_on_complex(){
    let data= DataInit {
        num1 : Complex::new(2,4),
        num2 : Complex::new(3,6),
    };

    println!("Addition is: {}\n",add(&data));
    println!("Substraction is: {}\n",sub(&data));
    println!("Multiplication is: {}\n",mul(&data));
}

fn add(data: &DataInit) ->Complex<i32>{
    data.num1+data.num2
}

fn sub(data: &DataInit) ->Complex<i32>{
    data.num1-data.num2
}

fn mul(data: &DataInit) ->Complex<i32>{
    data.num1*data.num2
}



struct Student{
    name: String,
    roll_no: i32,
    score_of_each_subject: Score,
    department: String,
    school: String,
}

struct Score{
    hindi: i32,
    english: i32,
    maths: i32,
    science: i32,
}


fn opration_on_struct(){
    fn new() -> Student{      //Q2_1//  To initialize Student objects.

        let user = Student {
            name: String::from("john"),
            department: String::from("tech"),
            school: String::from("zcoer"),
            roll_no: 1,
            score_of_each_subject: Score{
                hindi:24,
                english: 50,
                maths: 31,
                science: 70,
            },
        };
        return user;
    }

    let sr=new();   //Q2_2//    To get average of all scores.

    println!("Details of {} are as follows:\n",sr.name);
    println!("Name: {}\n Roll-No: {}\n Department: {}\n School: {}\n ",sr.name,sr.roll_no,sr.department,sr.school);

    let src=sr.score_of_each_subject;
    let avg=src.hindi+src.english+src.maths+src.science;
    println!("Average of 4 subject is: {}\n",avg/4);

    //Q2_3//    Add numbers to student’s subject score if score is less than 35.
    println!("Marks required to pass in Hindi is {}\n",src.marks_required_in_hindi());
    println!("Marks required to pass in Maths is {}\n",src.marks_required_in_maths());

}

impl Score {
    fn marks_required_in_hindi(&self) -> i32 {
        let marks_required=35-self.hindi;
        return marks_required;      //return difference
    }
    fn marks_required_in_maths(&self) -> i32 {
        let marks_required=35-self.maths;
        return marks_required;      //return difference
    }
}