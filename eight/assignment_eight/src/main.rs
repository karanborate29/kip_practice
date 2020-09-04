use assignment_eight::even_odd::even_num;
use assignment_eight::ip::classes;
use assignment_eight::quadrant::point;

fn main() {
    println!("Assignment Eight\n");

    even_num::even_odd_array(); //Question 1
    point::quadrant_matching(); //Question 2 -> A
    classes::search_ip(); //Question 2 -> B

    //Q:3: Practice code of the error handling and Automated tests.
    //Ans: https://github.com/karanborate29/Error_Handling/blob/master/error_testing/src/main.rs
}
