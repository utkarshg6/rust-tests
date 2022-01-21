use mathematical_functions::sum;

#[test]
fn sum_positives() {
    assert_eq!(sum(3, 4), 3 + 4)
}

#[test]
fn sum_negatives() {
    assert_eq!(sum(-2, -3), -2 + (-3))
}

#[test]
fn sum_positive_negative() {
    assert_eq!(sum(2, -3), 2 + (-3))
}

#[test]
fn sum_negative_positive() {
    assert_eq!(sum(-2, 3), -2 + 3)
}