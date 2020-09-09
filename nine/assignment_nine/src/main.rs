fn main() {
    let min = Data {
        num1: 'a',
        num2: 'z',
    };

    println!("Minimum is: {}", min.get_min());

    let mut input_array: [i32; 10] = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];
    sort(&mut input_array);
    println!("Sorted: {:?}", input_array);
}

struct Data<T> {
    num1: T,
    num2: T,
}

impl<T: PartialOrd> Data<T> {
    fn get_min(&self) -> &T {
        let minimum: &T;
        if &self.num1 < &self.num2 {
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

        // for i in 1..length {
        //     if input_array[i - 1] > input_array[i] {
        //         input_array.swap(i - 1, i);
        //         flag = true;
        //     }
        // }
        for i in 1..length {
            if input_array[i - 1] > input_array[i] {
                let tmp:T = input_array[i - 1].clone();
                input_array[i - 1] = input_array[i].clone();
                input_array[i] = tmp;

                flag = true;
            }
        }
        length = length - 1;
    }
}