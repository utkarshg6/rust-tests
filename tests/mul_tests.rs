use mathematical_functions::mul;

#[test]
fn mul_positives() {
    assert_eq!(mul(3, 4), 3 * 4)
}

#[test]
fn mul_negatives() {
    assert_eq!(mul(-2, -3), -2 * (-3))
}

#[test]
fn mul_positive_negative() {
    assert_eq!(mul(2, -3), 2 * (-3))
}

#[test]
fn mul_negative_positive() {
    assert_eq!(mul(-2, 3), -2 * 3)
}