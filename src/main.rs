// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {    
    let mut cc: Vec<u32> = cc_number
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10))
        .filter(|o| o.is_some() )
        .map(|o| o.unwrap())
        .collect();

    if cc.len() < 2 {
        return false;
    }
    
    for n in cc.iter_mut().rev().skip(1).step_by(2) {
        *n *= 2;
        while *n > 10 {
            *n = *n / 10 + *n % 10
        }
    }
    println!("cc after alternating doubling: {:?}", cc);

    let sum: u32 = cc.iter().sum();
    println!("sum: {}", sum);
    if sum % 10 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}