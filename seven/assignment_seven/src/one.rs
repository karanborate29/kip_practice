//Q1-> a
pub fn search_given_pattern() {
    let input_string: String = String::from("Pankaj Chaudhary");
    let pattern: String = String::from("Cha");
    let lowercase_input: String = input_string.to_lowercase();
    let lowercase_pattern: String = pattern.to_lowercase();
    if lowercase_pattern.len() == 0 {       //If pattern is absent
        println!("Not Found");
    } else {
        let index: usize = search_pattern(&lowercase_input, &lowercase_pattern);
        if index == lowercase_input.len() {
            println!("Not Found");
        } else {
            println!("{} found at index {}", pattern, index);       //Print the Index of Pattern found in string
        }
    }
}

fn search_pattern(input_string: &str, pattern: &str) -> usize {
    let string_vec: Vec<char> = input_string.chars().collect();
    let pattern_vec: Vec<char> = pattern.chars().collect();

    let mut _input_index: usize = 0;        //To maintain index of Input ie String_Vec
    let mut pattern_index: usize = 0;        //To maintain index of pattern ie pattern_Vec
    let firstocc: usize;

    while _input_index != string_vec.len() && string_vec[_input_index] != '\0' {
        while _input_index != string_vec.len()
            && string_vec[_input_index] != pattern_vec[0]
            && string_vec[_input_index] != '\0'
        {
            _input_index += 1;
        }
        firstocc = _input_index;                    //To maintain first index of pattern found
        while pattern_index != pattern_vec.len()
            && string_vec[_input_index] == pattern_vec[pattern_index]
            && string_vec[_input_index] != '\0'
            && pattern_vec[pattern_index] != '\0'
        {
            _input_index += 1;
            if (pattern_index + 1) != pattern_vec.len() {
                pattern_index += 1;
            } else {
                return firstocc;        //To maintain first index of pattern found
            }
        }
        _input_index = firstocc + 1;
        break;
    }
    string_vec.len()            //If Nothing found return Length of String Given as input(Unary Operator(-1) is not possible here)
}

//Q1-> b
pub fn substring_generation() {
    let input_str = "abcd";
    printsub(&input_str);
}

fn printsub(input_str: &str) {
    let string_vec: Vec<char> = input_str.chars().collect();
    let n: usize = string_vec.len();
    let len: usize = 1;

    for len in len..=n {        //To print upto length od string
        let i: usize = 0;

        for i in i..=n - len {      //To print from 'o'th index upto total length - No. of loops encountered
            let j: usize = i + len - 1;
            let print_char: usize = i;

            for print_char in print_char..=j {
                print!("{}", string_vec[print_char]);   //Finally to print each char
            }
            println!();     //New Line
        }
    }
}
