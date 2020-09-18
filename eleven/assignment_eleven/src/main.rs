use crate::List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::List::{Cons, Nil};
    use crate::{first_repeated, nth_element, odd_number, second_repeated, List};

    #[test]
    fn first_repeated_success() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        let result: i32 = first_repeated(&list);
        assert_eq!(21, result);
    }

    #[test]
    fn first_repeated_fail() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        let result: i32 = first_repeated(&list);
        assert_eq!(21, result);
    }
    #[test]
    fn second_repeated_success() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        let result: i32 = second_repeated(&list);
        assert_eq!(5, result);
    }

    #[test]
    fn second_repeated_fail() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        let result: i32 = second_repeated(&list);
        assert_eq!(5, result);
    }
    #[test]
    fn nth_element_success() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        let search: i32 = 3;
        let result: i32 = nth_element(&list, search);
        assert_eq!(4, result);
    }

    #[test]
    fn nth_element_fail() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        let search: i32 = 7;
        let result: i32 = nth_element(&list, search);
        assert_eq!(4, result);
    }
    #[test]
    fn odd_number_success() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        let odd_num: i32 = 3;
        let result: i32 = odd_number(&list, odd_num);
        assert_eq!(3, result);
    }

    #[test]
    fn odd_number_fail() {
        let list = List::Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        let odd_num: i32 = 5;
        let result: i32 = odd_number(&list, odd_num);
        assert_eq!(3, result);
    }
}

fn main() {}

fn first_repeated(list: &List) -> i32 {
    let vec: Vec<i32> = Vec::new();
    let result: i32 = first_recursive(&list, vec);
    result
}

fn first_recursive(list: &List, mut vec: Vec<i32>) -> i32 {
    match list {
        Nil => -1,
        Cons(item, remaining) => {
            if vec.contains(&item) {
                *item
            } else {
                vec.push(item.clone());
                first_recursive(remaining, vec)
            }
        }
    }
}

fn second_repeated(list: &List) -> i32 {
    let vec: Vec<i32> = Vec::new();
    let count: i32 = 0;
    let result: i32 = second_recursive(&list, vec, count);
    result
}

fn second_recursive(list: &List, mut vec: Vec<i32>, mut count: i32) -> i32 {
    match list {
        Nil => -1,
        Cons(item, remaining) => {
            if vec.contains(&item) {
                count += 1;
                if count != 2 {
                    second_recursive(remaining, vec, count)
                } else {
                    *item
                }
            } else {
                vec.push(item.clone());
                second_recursive(remaining, vec, count)
            }
        }
    }
}

fn nth_element(list: &List, search: i32) -> i32 {
    let count: i32 = 0;
    let item: i32 = search_element(&list, count, search);
    item
}

fn search_element(list: &List, mut count: i32, search: i32) -> i32 {
    match list {
        Nil => -1,
        Cons(item, remaining) => {
            if count == search {
                *item
            } else {
                count += 1;
                search_element(remaining, count, search)
            }
        }
    }
}

fn odd_number(list: &List, odd_num: i32) -> i32 {
    let count: i32 = 0;
    let item: i32 = search_oddnum(&list, count, odd_num);
    item
}

fn search_oddnum(list: &List, mut count: i32, odd_num: i32) -> i32 {
    match list {
        Nil => -1,
        Cons(item, remaining) => {
            if item % 2 != 0 {
                count += 1;
                if count == odd_num {
                    *item
                } else {
                    search_oddnum(remaining, count, odd_num)
                }
            } else {
                search_oddnum(remaining, count, odd_num)
            }
        }
    }
}
