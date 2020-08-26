//Assignment_four
fn main() {
    println!("Hello, world!\n");

    point_matching();
    search_ip();
}
enum Class {
    ClassA(u8, u8, u8, u8),
    ClassB(u8, u8, u8, u8),
    ClassC(u8, u8, u8, u8),
    ClassD(u8, u8, u8, u8),
}

fn search_ip() {
    let input: [(u8, u8, u8, u8); 4] = [
        (192, 0, 1, 1),
        (230, 45, 6, 7),
        (198, 5, 6, 4),
        (170, 45, 23, 45),
    ];

    for ip in &input {
        search_class(ip);
    }

    fn search_class(ip: &(u8, u8, u8, u8)) {
        let point = ip;

        let class_a = Class::ClassA(192, 0, 1, 1);
        let class_b = Class::ClassB(230, 45, 6, 7);
        let class_c = Class::ClassC(198, 5, 6, 4);
        let class_d = Class::ClassD(170, 45, 23, 45);

        match point {

            class_a => println!("{:?}\n", point),
            class_b => println!("{:?}\n", point),
            class_c => println!("{:?}\n", point),
            class_d => println!("{:?}\n", point),
            _ => println!("Invalid Class"),
        }
    }
}





fn point_matching() {
    let input: [(i32, i32); 5] = [(2, 1), (1, 2), (-2, -3), (5, 6), (4, -4)];

    for element in &input {
        search_quadrant(element);
    }

    fn search_quadrant(element: &(i32, i32)) {
        let point :&(i32,i32) = element;
        match point {
            (2, 1) => println!("point {:?} lies in First Quadrant", point),
            (1, 2) => println!("point {:?} lies in First Quadrant", point),
            (-2, -3) => println!("point {:?} lies in Third Quadrant", point),
            (5, 6) => println!("point {:?} lies in First Quadrant", point),
            (4, -4) => println!("point {:?} lies in Second Quadrant", point),
            _ => println!("anything"),
        }
    }
}
