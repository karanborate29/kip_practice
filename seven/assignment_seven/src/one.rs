
//Q1-> a
pub fn search_given_pattern(){
    let input:String=String::from("Pankaj Chaudhary");
    let pattern: String=String::from("ank");
    let inputt=input.to_lowercase();
    let patternn=pattern.to_lowercase();
    if patternn.len()==0 {
        println!("Not Found");
    }else {
        let index=search_pattern(&inputt,&patternn);
        if index==inputt.len() {
            println!("Not Found");
        }else {
            println!("{} found at index {}",pattern,index);
        }
    }
}


fn search_pattern(input:&str,pattern:&str) -> usize {

    let input_array: Vec<char>=input.chars().collect();
    let pattern_array: Vec<char>=pattern.chars().collect();

    let mut index_of_input=0;let mut index_of_pattern=0;let firstocc;

    while index_of_input!=input_array.len() && input_array[index_of_input]!= '\0' {


        while index_of_input!=input_array.len() && input_array[index_of_input]!=pattern_array[0] && input_array[index_of_input]!='\0' {
            index_of_input=index_of_input+1;
        }
        firstocc=index_of_input;
        while index_of_pattern != pattern_array.len() && input_array[index_of_input] == pattern_array[index_of_pattern] && input_array[index_of_input] != '\0' && pattern_array[index_of_pattern] != '\0' {
            index_of_input=index_of_input+1;
            if (index_of_pattern+1) != pattern_array.len(){
                index_of_pattern=index_of_pattern+1;
            }else{
                return firstocc;
            }
        }
        index_of_input=firstocc+1;
        break;
    }
    return input_array.len()
}

//Q1-> b
pub fn substring_generation(){

    let input_str="pa";
    printsub(&input_str);
}

fn printsub(input_str:&str){
    let input_array: Vec<char>=input_str.chars().collect();
    let n=input_array.len();
    let len=1;

    for len in len..=n{
        let i=0;

        for  i in i..=n-len  {
            let j=i+len-1;
            let k=i;

            for k in k..=j {
                print!("{}",input_array[k]);
            }
            println!("");
        }
    }
}
