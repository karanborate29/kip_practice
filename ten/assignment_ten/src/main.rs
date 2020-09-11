use std::collections::HashMap;

#[cfg(test)]
mod tests {

    use super::duplicate;
    use super::first_even;
    use super::get_rev;
    use super::is_palindrome;
    use super::sum_conditional;
    use std::collections::HashMap;

    #[test]
    fn sum_conditional_success() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("anurag", 24);
        map.insert("daniel", 23);
        map.insert("anushka", 30);
        let str: &str = "anu";
        let total: i32 = sum_conditional(map, str);
        assert_eq!(54, total);
    }

    #[test]
    fn sum_conditional_fail() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("anurag", 24);
        map.insert("daniel", 23);
        map.insert("anushka", 30);
        let str: &str = "kar";
        let total: i32 = sum_conditional(map, str);
        assert_ne!(54, total);
    }

    #[test]
    fn palindrome_success() {
        let list: Vec<i32> = vec![1, 2, 3, 2, 1];
        let result: bool = is_palindrome(list);
        assert_eq!(true, result);
    }

    #[test]
    fn palindrome_fail() {
        let list: Vec<i32> = vec![1, 2, 3, 2, 5];
        let result: bool = is_palindrome(list);
        assert_ne!(true, result);
    }
    #[test]
    fn first_even_success() {
        let even_odd: Vec<i32> = vec![1, 21, 3, 4, 5];
        let first_even_num = first_even(even_odd);
        assert_eq!(4, first_even_num);
    }
    #[test]
    fn first_even_fail() {
        let even_odd: Vec<i32> = vec![1, 21, 3, 7, 5];
        let first_even_num = first_even(even_odd);
        assert_ne!(4, first_even_num);
    }
    #[test]
    fn reverse_success() {
        let original_list: Vec<i32> = vec![1, 2, 3, 4, 5];
        let _reverse: Vec<i32> = get_rev(original_list);
        assert_eq!(vec![5, 4, 3, 2, 1], _reverse);
    }
    #[test]
    fn reverse_fail() {
        let original_list: Vec<i32> = vec![1, 2, 3, 4, 5];
        let _reverse: Vec<i32> = get_rev(original_list);
        assert_ne!(vec![1, 2, 3, 4, 5], _reverse);
    }
    #[test]
    fn duplicate_success() {
        let single_vec: Vec<i32> = vec![1, 2, 3, 3, 4];
        let _dup_vec: Vec<i32> = duplicate(single_vec);
        assert_eq!(vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 4], _dup_vec);
    }
    #[test]
    fn duplicate_fail() {
        let single_vec: Vec<i32> = vec![];
        let _dup_vec: Vec<i32> = duplicate(single_vec);
        assert_ne!(vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 4], _dup_vec);
    }
}

fn main() {}

fn sum_conditional(map: HashMap<&str, i32>, str: &str) -> i32 {
    let mut total: i32 = 0;

    for (key, value) in &map {
        if key.contains(str) {
            total += value;
        }
    }
    total
}

fn is_palindrome(list: Vec<i32>) -> bool {
    let mut flag: bool = true;
    let mut i: usize = 0;
    let mut j: usize = list.len() - 1;
    let vec_half: usize = list.len() / 2;

    while i <= vec_half && !list.is_empty() {
        if list[i] != list[j] {
            flag = false;
            break;
        } else {
            i += 1;
            j -= 1;
        }
    }
    flag
}

fn get_rev(mut original_list: Vec<i32>) -> Vec<i32> {
    let mut end: usize = original_list.len() - 1;
    let mut start: usize = 0;
    let mut temp: i32;

    while start < end {
        temp = original_list[start];
        original_list[start] = original_list[end];
        original_list[end] = temp;
        start += 1;
        end -= 1;
    }
    original_list
}

fn first_even(even_odd: Vec<i32>) -> i32 {
    let mut even_num = 0;
    let mut count = 0;
    let end = even_odd.len() - 1;

    while count < end {
        if even_odd[count] % 2 == 0 {
            even_num = even_odd[count];
            break;
        } else {
            count += 1;
        }
    }
    even_num
}

fn duplicate(single_vec: Vec<i32>) -> Vec<i32> {
    let mut dup_vec: Vec<i32> = Vec::new();
    if single_vec.is_empty() {
        return dup_vec;
    }
    let end: usize = single_vec.len() - 1;
    let mut start: usize = 0;

    while start <= end {
        let element: i32 = single_vec[start];
        dup_vec.push(element);
        dup_vec.push(element);
        start += 1;
    }
    dup_vec
}
