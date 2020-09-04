use crate::ip::classes::Class::{ClassA, ClassB, ClassC, ClassD, ClassE};

#[derive(Debug)]
enum Class {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
}

pub fn search_ip() {
    let input: [(i32, i32, i32, i32); 6] = [
        (120, 45, 23, 45),
        (192, 0, 1, 1),
        (230, 45, 6, 7),
        (198, 5, 6, 4),
        (170, 45, 23, 45),
        (251, 45, 23, 45),
    ];

    for ip in &input {
        let octet = ip.0;

        match octet {
            1..=127 => {
                let res = Class::search_class(&Class::ClassA, ip);
                println!("{:?}::{:?} ", res, ip);
            }
            128..=191 => {
                let res = Class::search_class(&Class::ClassB, ip);
                println!("{:?}::{:?} ", res, ip);
            }
            192..=223 => {
                let res = Class::search_class(&Class::ClassC, ip);
                println!("{:?}::{:?} ", res, ip);
            }
            224..=239 => {
                let res = Class::search_class(&Class::ClassD, ip);
                println!("{:?}::{:?} ", res, ip);
            }
            240..=255 => {
                let res = Class::search_class(&Class::ClassE, ip);
                println!("{:?}::{:?} ", res, ip);
            }
            _ => {}
        }
    }
}

impl Class {
    fn search_class(&self, _ip: &(i32, i32, i32, i32)) -> Class {
        match &self {
            Class::ClassA => ClassA,
            Class::ClassB => ClassB,
            Class::ClassC => ClassC,
            Class::ClassD => ClassD,
            Class::ClassE => ClassE,
        }
    }
}
