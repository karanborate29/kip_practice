use std::io;

#[derive(Debug)]
pub struct Library {
    accession_num: i32,
    author: String,
    title: String,
}

fn main() {
    let vec: Vec<Library> = Vec::new();
    choice(vec);
}

pub fn choice(vec: Vec<Library>) {
    println!("****library Menu****\n");
    println!("1: Add a new book\n");
    println!("2: Display book information\n");
    println!("3: Display all the books in the library of a particular author\n");
    println!("4: Display the number of books of a particular title\n");
    println!("5: Display the total number of books in the library\n");
    println!("Enter ur choice:");
    let mut inputt: String = String::new();
    io::stdin().read_line(&mut inputt).unwrap();
    let input: i32 = inputt.trim().parse().unwrap();
    match input {
        1 => Library::add(vec),
        2 => Library::display_book(vec),
        3 => Library::disp_book_author(vec),
        4 => Library::disp_book_title(vec),
        5 => Library::books_total(vec),
        _ => {
            println!("Invalid Input");
        }
    }
}

impl Library {
    fn add(mut vec: Vec<Library>) {
        println!("Adding A Book");
        println!("Enter accession_number");
        let mut accession_num: String = String::new();
        io::stdin().read_line(&mut accession_num).unwrap();
        let accession: i32 = accession_num.trim().parse().unwrap();
        println!("Enter author");
        let mut authorr: String = String::new();
        io::stdin().read_line(&mut authorr).unwrap();
        println!("Enter title of book");
        let mut tittle: String = String::new();
        io::stdin().read_line(&mut tittle).unwrap();

        let mut count: i32 = 0;
        for item in &vec {
            if item.accession_num == accession {
                count += 1;
            }
        }
        if count == 0 {
            vec.push(Library {
                accession_num: accession,
                author: authorr,
                title: tittle,
            });
            choice(vec);
        } else {
            println!("{} Books with same Accession Number are present in Library. Plz Enter different Accession Number",count);
        }
    }

    fn display_book(vec: Vec<Library>) {
        if vec.is_empty() {
            println!("Library Is Empty. Add Books to view");
        }
        for item in &vec {
            println!(
                "Accession Number: {:?} Author: {:?} Title: {:?}",
                item.accession_num, item.author, item.title
            );
        }
        choice(vec);
    }

    fn disp_book_author(vec: Vec<Library>) {
        println!("Enter the author to search his Books:");
        let mut author_name = String::new();
        io::stdin().read_line(&mut author_name).unwrap();
        for item in &vec {
            if item.author == author_name {
                println!("{:?}", item.title);
            }
        }
        choice(vec);
    }

    fn disp_book_title(vec: Vec<Library>) {
        println!("Enter the title of Book to find count:");
        let mut title_name: String = String::new();
        io::stdin().read_line(&mut title_name).unwrap();
        let mut count: i32 = 0;
        for item in &vec {
            if item.title == title_name {
                count += 1;
            }
        }
        println!(
            "Number of Books present in Library with Title {} is: {}",
            title_name, count
        );
        choice(vec);
    }
    fn books_total(vec: Vec<Library>) {
        let count: usize = vec.len();
        println!("Number of Books present in Library: {}", count);
        choice(vec);
    }
}
