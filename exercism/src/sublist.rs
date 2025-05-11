use std::env;

fn main() {
    //unsafe { env::set_var("RUST_BACKTRACE", "1") };

    empty_lists();
    empty_list_within_non_empty_list();
    non_empty_list_contains_empty_list();
    list_equals_itself();
    different_lists();
    false_start();
    at_start_of_superlist();
    first_list_missing_element_from_second_list();
    order_matters_to_a_list();
    at_end_of_superlist();
}

fn at_end_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn order_matters_to_a_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[3, 2, 1];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn empty_lists() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}

fn empty_list_within_non_empty_list() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn non_empty_list_contains_empty_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn list_equals_itself() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}

fn different_lists() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[2, 3, 4];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn false_start() {
    let list_one: &[i32] = &[1, 2, 5];
    let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn at_start_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn first_list_missing_element_from_second_list() {
    let list_one: &[i32] = &[1, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    if first_list_len == second_list_len {
        if first_list_len == 0 {
            return Comparison::Equal;
        } else if let Some(value) = check_sublist(first_list, second_list) {
            return value;
        }
        return Comparison::Equal;
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() {
        return Comparison::Superlist;
    } else if first_list_len < second_list_len {
        if let Some(value) = check_sublist(first_list, second_list) {
            return value;
        }
        return Comparison::Sublist;
    }
    if let Some(value) = check_sublist(second_list, first_list) {
        return value;
    }
    Comparison::Superlist
}

fn check_sublist(first_list: &[i32], second_list: &[i32]) -> Option<Comparison> {
    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    for (idx, val) in second_list.iter().enumerate() {
        if *val == first_list[0] {
            if idx + first_list_len > second_list_len {
                break
            }

            let sublist = &second_list[idx..(idx + first_list_len)];
            if first_list == sublist {
                return None;
            }
        }
    }

    Some(Comparison::Unequal)
}