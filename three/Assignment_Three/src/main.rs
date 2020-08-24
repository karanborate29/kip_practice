//assignment_three

use std::io::{self, Write};

fn main(){

    let arr = [1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210, 222];
    let target: i32 = 47;

    linearsearch(arr,target);   //Q1(a)_Calling Linear_Search
    binarysearch(arr, target);  //Q1(b)_Calling Binary_Search

    let mut nums = vec![133, 10, 210, 59, 47, 63, 75, 88, 99, 120, 1, 155, 162, 176, 222, 199, 200, 20, 107];

    let l = nums.len();
    merge_sort(&mut nums, 0, l - 1);    //Q2_Merge_Sort

    println!("Displaying Sorted Array");
    for f in nums {
        print!("{} ", f);
    }
    println!("");
    io::stdout().flush().unwrap();

    let tup=(04,1999);
    leap(tup);  //Q3_Calling Leap Year
}

pub fn linearsearch(arr: [i32;20],target_value:i32) {
        let size = arr.len();
        linearsearchrecursive(arr, 0, size as i32, target_value);
}

pub fn linearsearchrecursive(arr: [i32;20], position:i32, length:i32, target_value:i32) {
    let my_size = length-1;
    if my_size < -1 {
        println!("In liner search, found empty array");
    }
    if arr[position as usize] == target_value {
        println!("In liner search, {} at position {}",target_value, (position+1));
        return ();
    }
    if arr.len() != my_size as usize && arr[mySize as usize] == target_value {
        println!("In liner search, {} at position {}",target_value, (my_size+1));
        return ();
    }
    let size = arr.len()-1;
    linearsearchrecursive(arr, position+1, (size - 1) as i32, target_value);
}

pub fn binarysearch(arr: [i32;20],target_value:i32){
    let size = arr.len();
    binarysearchrecursive(arr, 0, size as i32, target_value);

}

pub fn binarysearchrecursive(arr: [i32;20], start:i32, len: i32, target_value: i32) {
    let mut low: i32 = start ;
    let mut high: i32 = len - 1;

        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = arr[mid_index];
        if val == target_value {
            println!("In Binary Search, {} found at index {}", target_value, mid_index);
            println!(" ");
            return();
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
            binarysearchrecursive(arr, low , arr.len() as i32, target_value);
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
            binarysearchrecursive(arr, 0,  high, target_value);
        }


}



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