//assignment_three
use std::io::{self, Write};

fn main(){

    let arr = [
        1, 10, 20, 47, 59, 63, 75, 88, 99,
        107, 120, 133, 155, 162, 176, 188,
        199, 200, 210, 222
    ];
    let target: i32 = 47;

    linearsearch(&arr,&target);
    binarysearch(&arr, arr.len(), &target);

    let mut nums = vec![133, 10, 210, 59, 47, 63, 75, 88, 99
                        , 120, 1, 155, 162, 176, 222,
                        199, 200, 20, 107];

    let l = nums.len();
    merge_sort(&mut nums, 0, l - 1);
    println!("Implementing Merge Sort");
    for f in nums {
        print!("{} ", f);
    }
    println!("");
    io::stdout().flush().unwrap();

    let tup=(04,1999);
    leap(tup);
}

pub fn linearsearch(arr: &[i32],target_value:&i32) {

    println!("In Linear Search");
    for  val in arr{
            if val==target_value {
                println!("{} is present",target_value);
        }

    }

}



pub fn binarysearch(a: &[i32], len: usize, target_value: &i32) {
    println!("");
    println!("In Binary Search");
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &a[mid_index];
        if val == target_value {
            println!("{} found at index {}", target_value, mid_index);
            println!(" ");
            break;
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
        }
    }

}


/*use std::io::{self, Write};
fn main() {
    let mut nums = vec![133, 10, 210, 59, 47, 63, 75, 88, 99
                        , 120, 1, 155, 162, 176, 222,
                        199, 200, 20, 107];

    let l = nums.len();
    merge_sort(&mut nums, 0, l - 1);
    println!("Implementing Merge Sort");
    for f in nums {
        print!("{} ", f);
    }
    io::stdout().flush().unwrap();
}
*/
pub fn merge_sort(a: &mut Vec<i32>, b: usize, e: usize) {
    if b < e {
        let m = (b+e)/2;
        merge_sort(a, b, m);
        merge_sort(a, m+1, e);
        merge(a, b, m, e);
    }
}
fn merge(a: &mut Vec<i32>, b: usize, m:usize, e:usize) {
    let mut left = a[b..m+1].to_vec();
    let mut right = a[m+1..e+1].to_vec();
    left.reverse();
    right.reverse();
    for k in b..e + 1 {
        if left.is_empty() {
            a[k] = right.pop().unwrap();
            continue;
        }
        if right.is_empty() {
            a[k] = left.pop().unwrap();
            continue;
        }
        if right.last() < left.last() {
            a[k] = right.pop().unwrap();
        }
        else {
            a[k] = left.pop().unwrap();
        }
    }
}

pub fn leap(data:(i32,i32)){

    let year=data.1;
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0{
        println!("");
        println!("{ } is a leap year",year);
    }else {
        println!("");
        println!("{ } not a leap year",year);
    }
}