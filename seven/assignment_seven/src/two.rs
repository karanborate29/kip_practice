
//Q2
pub fn desired_output(){
      let str1="jjdhid";
      let str2="ikjhjk";
      let str3="rtysgi";

    println!("Output: {}",get_desired_output(str1,str2,str3));
    }

fn get_desired_output(str1:&str,str2:&str,str3:&str) -> String{

    let str_one: Vec<char>=str1.chars().collect();
    let str_two: Vec<char>=str2.chars().collect();
    let str_three: Vec<char>=str3.chars().collect();
    let mut count=1;    let mut output=String::new();
    let mut smaller=String::new(); let mut greater=String::new();

        while count <= str_one.len() {

            for char1 in &str_one{

                for char2 in &str_two{

                    for char3 in &str_three{

                        if count%2==0 {
                            //smaller
                            if char1 < char2 && char1 < char3 {
                                smaller.push(*char1);
                            }else if char2<char3 {
                                smaller.push(*char2);
                            }else {
                                smaller.push(*char3)
                            }
                            output.push_str(&smaller);
                        }else {
                            //greater
                            if char1>char2 {
                                if char1>char3 {
                                    greater.push(*char1);
                                }else {
                                    greater.push(*char3);
                                }
                            }else if char2>char3 {
                                greater.push(*char2);
                            }else {
                                greater.push(*char3);
                            }
                            output.push_str(&greater);
                        }
                    }
                }
                count=count+1;
            }
        }
    output

}

