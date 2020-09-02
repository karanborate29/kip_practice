//Assignment_six
fn main() {
    quadrant_matching(); //Q1 => Find Point lies On Which Quadrant
    println!();
    search_ip(); //Q2 => Find Ip Belong To Which Class
}

enum Position {
    First,
    Second,
    Third,
    Fourth,
    X,
    Y,
}

//Calling Q1 Function
fn quadrant_matching() {
    let input: [(i32, i32); 5] = [(0, 2), (2, 0), (-2, -3), (5, 6), (4, -4)];

    for element in &input {
        if element.0 > 0 && element.1 > 0 {
            Position::search_quadrant(&Position::First, element);
        }
        if element.0 < 0 && element.1 > 0 {
            Position::search_quadrant(&Position::Second, element);
        }
        if element.0 < 0 && element.1 < 0 {
            Position::search_quadrant(&Position::Third, element);
        }
        if element.0 > 0 && element.1 < 0 {
            Position::search_quadrant(&Position::Fourth, element);
        }
        if element.0 > 0 && element.1 == 0 {
            Position::search_quadrant(&Position::X, element);
        }
        if element.0 == 0 && element.1 > 0 {
            Position::search_quadrant(&Position::Y, element);
        }
    }
}
//Implementing Q1 Static Method "search_quadrant" using Enum
impl Position {
    fn search_quadrant(&self, element: &(i32, i32)) {
        match &self {
            Position::First => println!(
                "First Quadrant Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            Position::Second => println!(
                "Second Quadrant Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            Position::Third => println!(
                "Third Quadrant Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            Position::Fourth => println!(
                "Fourth Quadrant Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            Position::X => println!(
                "Positive X Axis Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            Position::Y => println!(
                "Positive Y Axis Abscissa({:?}), Ordinate({:?})",
                element.0, element.1
            ),
            _ => println!("Wrong Input\n"),
        }
    }
}

enum Class {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
}
//Calling Q2 Function
fn search_ip() {
    let input: [(i32, i32, i32, i32); 6] = [
        (120, 45, 23, 45),
        (192, 0, 1, 1),
        (230, 45, 6, 7),
        (198, 5, 6, 4),
        (170, 45, 23, 45),
        (251, 45, 23, 45),
    ];

    for ip in &input {
        let octet = ip.0; //No If/Else only pattern matching
        match octet {
            1..=127 => {
                Class::search_class(&Class::ClassA, ip);
            }
            128..=191 => {
                Class::search_class(&Class::ClassB, ip);
            }
            192..=223 => {
                Class::search_class(&Class::ClassC, ip);
            }
            224..=239 => {
                Class::search_class(&Class::ClassD, ip);
            }
            240..=255 => {
                Class::search_class(&Class::ClassE, ip);
            }
            _ => {}
        }
    }
}
//Implementing Q2 Static Method "search_class" using Enum
impl Class {
    fn search_class(&self, ip: &(i32, i32, i32, i32)) {
        match &self {
            Class::ClassA => println!("ClassA({:?})", ip),
            Class::ClassB => println!("ClassB({:?})", ip),
            Class::ClassC => println!("ClassC({:?})", ip),
            Class::ClassD => println!("ClassD({:?})", ip),
            Class::ClassE => println!("Reserved ClassE {:?}", ip),
            _ => println!("Wrong Input"),
        }
    }
}
