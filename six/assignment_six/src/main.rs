//Assignment_six
fn main() {
    println!("Hello, world!\n");

    quadrant_matching();            //Q1 => Find Point lies On Which Quadrant
    println!("");
    search_ip();                    //Q2 => Find Ip Belong To Which Class
}

enum Position{
    First,
    Second,
    Third,
    Fourth,
}

//Calling Q1 Function
fn quadrant_matching() {
    let input: [(i32, i32); 5] = [(2, 1), (2, -2), (-2, -3), (5, 6), (4, -4)];

    for element in &input {

        if element.0 > 0 && element.1 > 0{
            let get_pos=Position::First;
               Position::search_quadrant(&get_pos,element);
        }
        if element.0 < 0 && element.1 > 0{
            let get_pos=Position::Second;
            Position::search_quadrant(&get_pos,element);
        }
        if element.0 < 0 && element.1 < 0{
            let get_pos=Position::Third;
            Position::search_quadrant(&get_pos,element);
        }
        if element.0 > 0 && element.1 < 0{
            let get_pos=Position::Fourth;
            Position::search_quadrant(&get_pos,element);
        }
    }
}
//Implementing Q1 Static Method "search_quadrant" using Enum
impl Position{
    fn search_quadrant(pos:&Position,element: &(i32, i32)) {
        match pos {
            Position::First => println!("Position::First(Coordinate::Abscissa({:?}), Coordinate::Ordinate({:?})",element.0 ,element.1),
            Position::Second => println!("Position::Second(Coordinate::Abscissa({:?}), Coordinate::Ordinate({:?})",element.0 ,element.1),
            Position::Third => println!("Position::Third(Coordinate::Abscissa({:?}), Coordinate::Ordinate({:?})",element.0 ,element.1),
            Position::Fourth => println!("Position::Fourth(Coordinate::Abscissa({:?}), Coordinate::Ordinate({:?})",element.0 ,element.1),
            _ => println!("Wrong Input\n"),
        }
    }
}

enum Class {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
}
//Calling Q2 Function
fn search_ip() {

    let input: [(i32, i32, i32, i32); 5] = [(120, 45, 23, 45),
        (192, 0, 1, 1),
        (230, 45, 6, 7),
        (198, 5, 6, 4),
        (170, 45, 23, 45)
    ];

    for ip in &input {
        let octet=ip.0;             //No If/Else only pattern matching
        match octet {
            1..=127 => {
                let get_class=Class::ClassA;
                Class::search_class(&get_class,ip);
            },
            128..=191 => {
                let get_class=Class::ClassB;
                Class::search_class(&get_class,ip);
            },
            192..=223 => {
                let get_class=Class::ClassC;
                Class::search_class(&get_class,ip);
            },
            224..=239 => {
                let get_class=Class::ClassD;
                Class::search_class(&get_class,ip);
            },
            _ => {()},
        }
    }
}
//Implementing Q2 Static Method "search_class" using Enum
impl Class{
    fn search_class(get_class: &Class,ip: &(i32, i32, i32, i32)){
        match get_class {
            Class::ClassA =>  println!("Ip::ClassA({:?})",ip),
            Class::ClassB =>  println!("Ip::ClassB({:?})",ip),
            Class::ClassC =>  println!("Ip::ClassC({:?})",ip),
            Class::ClassD =>  println!("Ip::ClassD({:?})",ip),
            _ => println!("Wrong Input"),
        }
    }
}