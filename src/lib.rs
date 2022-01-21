pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn mul(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

/*
Why the move?


It's possible to locate unit tests outside the production-code files (although you'd still want
to tag them with #[cfg(test)]), but that's not the convention in Rust, and that's not the way we
do it in MASQ.

Unit tests go in the same file as the code they test.

That isn't to say we never have test/ directories. We do, but it's _integration_ tests that go in
the test/ directories. Integration tests don't have the same kind of tightly-coupled access to the
production code that unit tests do. For integration tests, the production code is processed into
a separate crate, and the integration tests are forced to interact with it only through the public
crate interface.

That's quite useful--for integration tests--but you don't want to try writing unit tests that way.
 */

#[cfg(test)]
mod tests {
    use super::*;

    /*
    Note:

    I understand that tests are fun to write and even more fun to watch pass. In an exercise like
    this, there's no reason not to write all the tests you want.

    However, in real life, unless you're making a specific rhetorical point, you want to confine
    yourself to tests that drive code. For example, you could drive either sum() or mul() by writing
    one of the tests below. You'd write the test, watch it fail, complete either sum() or mul(),
    watch the test succeed, and then...you're done.  You can write the other three tests if you
    want, but they'll pass immediately: they won't drive any new code. Tests that don't drive code
    are overhead: they increase the cost of refactoring, if you decide on a refactoring that requires
    changing tests. That might be justified if the extra tests communicate something important to a
    reader; otherwise usually not.
     */

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
}