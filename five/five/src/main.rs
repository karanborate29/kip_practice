//Assignment Five

fn main() {
    let num1 = ComplexStructure { real: 2, imag: 4 };
    let num2 = ComplexStructure { real: 3, imag: 6 };
    let addition: ComplexStructure = ComplexStructure::add(num1, num2); //Calling static Method "Addition"
    println!("\
        Addition: { }+{ }i",
             addition.real, addition.imag
    );

    let num1 = ComplexStructure { real: 2, imag: 4 };
    let num2 = ComplexStructure { real: 3, imag: 6 };
    let substraction: ComplexStructure = ComplexStructure::sub(num1, num2); //Calling static Method "Substraction"
    println!(
        "substraction: { } { }i",
        substraction.real, substraction.imag
    );

    let num1 = ComplexStructure { real: 2, imag: 1 };
    let num2 = ComplexStructure { real: 2, imag: 1 };
    let multiplication: ComplexStructure = ComplexStructure::mul(num1, num2); //Calling static Method "Multiplication"
    println!(
        "Multiplication: { } + { }i",
        multiplication.real, multiplication.imag
    );

    //Q2-> A
    let data: Student = Student::new(
        "john".to_string(),
        "tech".to_string(),
        "zcoer".to_string(),
        13,
        24,
        50,
        31,
        70,
    ); //To initialize Student objects.
       //Q2-> B
    println!("Avg: {}", Score::get_avg(&data.score_of_each_subject)); // To get Average
                                                                      //Q2-> C
    let _student_passed: Score = Student::pass_student(&data); // To Pass Student
                                                               //Q2-> D
    let student_two = Score {
        hindi: 80,
        english: 70,
        maths: 60,
        science: 50,
    };
    Score::compare_student(student_two, _student_passed); //Print difference of each subject’s score.
}

struct ComplexStructure {
    real: i32,
    imag: i32,
}

impl ComplexStructure {
    //Implementing static Method
    fn add(num1: ComplexStructure, num2: ComplexStructure) -> ComplexStructure {
        ComplexStructure {
            real: num1.real + num2.real,
            imag: num1.imag + num2.imag,
        } //Addition
    }
    fn sub(num1: ComplexStructure, num2: ComplexStructure) -> ComplexStructure {
        ComplexStructure {
            real: num1.real - num2.real,
            imag: num1.imag - num2.imag,
        } //Substraction
    }
    fn mul(num1: ComplexStructure, num2: ComplexStructure) -> ComplexStructure {
        ComplexStructure {
            real: num1.real * num2.real - num1.imag * num2.imag,
            imag: num1.real * num2.imag + num1.imag * num2.real,
        } //Multiplication
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

impl Student {
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
}

impl Score {
    //Implementing Method
    fn get_avg(&self) -> i32 {
        let avg:i32 = self.hindi + self.maths + self.english + self.science;
        let result: i32 = avg / 4;
        result
    }

    fn compare_student(student_two: Score, student_one: Score) {
        println!("Differences are as follows:");
        println!("Hindi: {}", student_two.hindi - student_one.hindi); //Hindi_Difference
        println!("Maths: {}", student_two.maths - student_one.maths); //Maths_Difference
        println!("English: {}", student_two.english - student_one.english); //English_Difference
        println!("Science: {}", student_two.science - student_one.science); //Science_Difference
    }
}
