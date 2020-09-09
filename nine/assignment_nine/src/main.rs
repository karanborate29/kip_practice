fn main() {
    println!("Assignment Nine");

    let min = Data {
        num1: 'a',
        num2: 'z',
    };
    println!("Minimum is: {}", min.get_min());           //Q1 -> A

    let mut input_array: [i32; 10] = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];            //Q1 -> B
    sort(&mut input_array);
    println!("Sorted: {:?}", input_array);

    for i in geometric_progression().take(15) {
                                                                         //Q2
        println!("> {}", i);
    }
}

struct Data<T> {
    num1: T,
    num2: T,
}

impl<T: PartialOrd> Data<T> {
    fn get_min(&self) -> &T {
        let minimum: &T;
        if self.num1 < self.num2 {
            minimum = &self.num1;
        } else {
            minimum = &self.num2;
        }
        &minimum
    }
}

fn sort<T: std::clone::Clone + PartialOrd>(input_array: &mut [T]) {
    let mut length: usize = input_array.len();
    let mut flag: bool = true;

    while flag {
        flag = false;

        for i in 1..length {
            if input_array[i - 1] > input_array[i] {
                let tmp: T = input_array[i - 1].clone();
                input_array[i - 1] = input_array[i].clone();
                input_array[i] = tmp;

                flag = true;
            }
        }
        length -= 1;
    }
}

struct GeometricSeries {
    first_num: i32,
    current_num: i32,
    ratio: i32,
}

// Implement `Iterator` for `GeometricSeries`.
impl Iterator for GeometricSeries {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let mut sum: i32 = 0;
        sum += self.first_num;
        self.first_num *= self.ratio;

        Some(sum)
    }
}

fn geometric_progression() -> GeometricSeries {
    GeometricSeries {
        first_num: 2,
        current_num: 0,
        ratio: 2,
    }
}
