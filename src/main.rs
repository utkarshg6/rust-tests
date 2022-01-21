#[allow(dead_code)]
fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[test]
fn test_sum_positives() {
    assert_eq!(sum(3, 4), 3 + 4)
}

#[test]
fn test_sum_negatives() {
    assert_eq!(sum(-2, -3), -2 + (-3))
}

#[test]
fn test_sum_positive_negative() {
    assert_eq!(sum(2, -3), 2 + (-3))
}

#[test]
fn test_sum_negative_positive() {
    assert_eq!(sum(-2, 3), -2 + 3)
}


fn main() {
    println!("This project contains tests. Please run `cargo test` to run the tests.");
}
