//Assignment Five
extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!\n");
    opration_on_complex(); //Q1
    opration_on_struct(); //Q2-> 1-> 2-> 3
}

struct DataInit {
    num1: num::Complex<i32>,
    num2: num::Complex<i32>,
}

fn opration_on_complex() {
    let data = DataInit {
        num1: Complex::new(2, 4),
        num2: Complex::new(3, 6),
    };

    println!("Addition is: {}\n", add(&data));
    println!("Substraction is: {}\n", sub(&data));
    println!("Multiplication is: {}\n", mul(&data));
}

fn add(data: &DataInit) -> Complex<i32> {
    data.num1 + data.num2 //Addition
}

fn sub(data: &DataInit) -> Complex<i32> {
    data.num1 - data.num2 //Substraction
}

fn mul(data: &DataInit) -> Complex<i32> {
    data.num1 * data.num2 //Multiplication
}

//Score Structure
struct Score {
    hindi: i32,
    english: i32,
    maths: i32,
    science: i32,
}

//Student Structure
struct Student {
    name: String,
    roll_no: i32,
    score_of_each_subject: Score,
    department: String,
    school: String,
}

//Calling_Second_Question
fn opration_on_struct() {
    let data: Student = Student::new(               //Static Method
        //New Method -> To initialize
        String::from("John"),
        "tech".to_string(),
        "zcoer".to_string(),
        13,
        24,
        50,
        31,
        70,
    );

    let avg = Score::get_avg(&data.score_of_each_subject); //get_average Method -> To get Average
    println!("{}", avg);
    let _student_passed: Score = pass_student(&data);
}

impl Student {
    //Implementing Method
    fn new(
        get_name: String,
        get_dept: String,
        get_school: String,
        get_roll: i32,
        get_hindi: i32,
        get_eng: i32,
        get_maths: i32,
        get_science: i32,
    ) -> Student {
        Student {
            name: get_name,
            department: get_dept,
            school: get_school,
            roll_no: get_roll,
            score_of_each_subject: Score {
                hindi: get_hindi,
                english: get_eng,
                maths: get_maths,
                science: get_science,
            },
        }
    }
}

impl Score {
    //Implementing Method
    fn get_avg(&self) -> i32 {
        let avg = self.hindi + self.maths + self.english + self.science;
        let result: i32 = avg / 4;
        result
    }
}

fn pass_student(data: &Student) -> Score {
    let mut hindi_pass = data.score_of_each_subject.hindi;
    if hindi_pass < 35 {
        hindi_pass = 35;
    }
    let mut english_pass = data.score_of_each_subject.english;
    if english_pass < 35 {
        english_pass = 35;
    }
    let mut maths_pass = data.score_of_each_subject.maths;
    if maths_pass < 35 {
        maths_pass = 35;
    }
    let mut science_pass = data.score_of_each_subject.science;
    if science_pass < 35 {
        science_pass = 35;
    }
    let updated: Score = Score {
        hindi: hindi_pass,
        english: english_pass,
        maths: maths_pass,
        science: science_pass,
    };
    updated
}
