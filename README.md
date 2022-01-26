# Rust Tests

This project contains tests in Rust, in a beginner-friendly manner.

To run tests, you can run the command:

```zsh
cargo test
```

Here's a [guide](https://github.com/utkarshg6/helpful/blob/master/Rust.md#rust-tests) that can be used as a reference for writing tests in Rust.

## A Step by Step guide of the Test Driven Development:

Step 1: **Don't** write any code, write tests first.

Step 2: Run the test, and see the test getting failed, to prove that some changes in the code are required to pass the test.

Step 3: Now, write code such that the failing test(s) passes.

Step 4: Once, you see the test(s) getting passed. Repeat the process from Step 1.

Note: Any code that doesn't write drives new code is an overhead and it causes the cost of refactoring. Refer [here](https://github.com/utkarshg6/rust-tests/commit/c8219592c26fc2fcc32b805e3670bd0666c2e235#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R33) for more explanation.

Thank you [Dan](https://github.com/dnwiebe) for explaining me the basics of Test Driven Development, and [Bert](https://github.com/bertllll) for showing me the Hands on experience of writing tests in Rust.
