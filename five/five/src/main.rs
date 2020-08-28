//Assignment Five

fn main() {
    println!("Hello, world!\n");
    opration_on_complex(); //Q1
    opration_on_struct(); //Q2-> 1-> 2-> 3
}

struct ComplexStruct {
    real: i32,
    imag: i32,
}

fn opration_on_complex() {
    let num1 = ComplexStruct { real: 2, imag: 4 };
    let num2 = ComplexStruct { real: 3, imag: 6 };
    ComplexStruct::operate(num1, num2); //Calling static Method
}

impl ComplexStruct {
    //Implementing static Method
    fn operate(num1: ComplexStruct, num2: ComplexStruct) {
        println!("{} + {}i ", num1.real + num2.real, num1.imag + num2.imag); //Addition
        println!("{} + {}i ", num1.real - num2.real, num1.imag - num2.imag); //Substraction
        println!("{} + {}i ", num1.real * num2.real, num1.imag * num2.imag); //Multiplication
    }
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
    let data: Student = Student::new(
        //Static Method
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

    let avg = Score::get_avg(&data.score_of_each_subject); // To get Average
    println!("Avg: {}", avg);
    let student_passed: Score = pass_student(&data); // To Pass Student

    Score::compare_student(&student_passed); //Print difference of each subject’s score.
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
    fn compare_student(student_one: &Score) {
        let student_two = Score {
            //Declaring Second Student
            hindi: 80,
            english: 70,
            maths: 60,
            science: 50,
        };
        println!("Differences are as follows:");
        println!(
            "Hindi: {}",
            student_two.hindi - student_one.hindi
        ); //Hindi_Difference
        println!(
            "Maths: {}",
            student_two.maths - student_one.maths
        ); //Maths_Difference
        println!(
            "English: {}",
            student_two.english - student_one.english
        ); //English_Difference
        println!(
            "Science: {}",
            student_two.science - student_one.science
        ); //Science_Difference
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
