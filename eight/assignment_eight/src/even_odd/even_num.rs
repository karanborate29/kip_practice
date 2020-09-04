use std::fmt::Error;

pub fn even_odd_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("{:?}", is_even(element)); //Handle Error if No is Odd Or Ok if No is Even
    }
    println!();
}

///Testing by passing Odd No. to Show Error
#[test]
fn even_odd_err() {
    println!("{:?}", is_even(&3));
}

///public function to with return type as Result<>
fn is_even(num: &i32) -> Result<&i32, Error> {
    if num % 2 == 0 {
        Ok(num)
    } else {
        Err(Error)
    }
}
