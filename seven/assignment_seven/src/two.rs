//Q2
pub fn desired_output() {
    let str1: &str = "jjdhid";
    let str2: &str = "ikjhjk";
    let str3: &str = "rtysgi";

    println!("Output: {}", get_desired_output(str1, str2, str3));
}

fn get_desired_output(str1: &str, str2: &str, str3: &str) -> String {
    let str_one: Vec<char> = str1.chars().collect();
    let str_two: Vec<char> = str2.chars().collect();
    let str_three: Vec<char> = str3.chars().collect();
    let mut count: usize = 0;
    let mut output: String = String::from("");

    while count < str_one.len() {
        let mut smaller: String = String::from(""); //To hold Smaller Char among three input Strings
        let mut greater: String = String::from(""); //To hold Greater Char among three input Strings

        let char1: char = str_one[count];
        let char2: char = str_two[count];
        let char3: char = str_three[count];

        if count % 2 == 0 {
            //If mod=0 then it means that index is even and smaller char will be substituted in output String
            //smaller
            if char1 < char2 && char1 < char3 {
                smaller.push(char1);
            } else if char2 < char3 {
                smaller.push(char2);
            } else {
                smaller.push(char3)
            }
            output.push(smaller.parse().unwrap());
        } else {
            //greater
            if char1 > char2 {
                if char1 > char3 {
                    greater.push(char1);
                } else {
                    greater.push(char3);
                }
            } else if char2 > char3 {
                greater.push(char2);
            } else {
                greater.push(char3);
            }
            output.push(greater.parse().unwrap());
        }
        count += 1; //Increase Count to decide loop will be done for Even or Odd Index
    }
    output //Output String
}
