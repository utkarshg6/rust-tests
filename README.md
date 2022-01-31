# Rust Tests

This project contains tests in Rust, in a beginner-friendly manner.

To run tests, you can run the command:

```zsh
cargo test
```

Here's a [guide](https://github.com/utkarshg6/helpful/blob/master/Rust.md#rust-tests) that can be used as a reference for writing tests in Rust.

## A Step by Step guide of the Test Driven Development:

Step 1: **Don't** write any code, write tests first. Elaboration: It's okay to frame out production data
structures and code patterns without tests; but they should contain no valid behavioral code. Use macros
like `unimplemented!()` or `todo!()` to hold places for code that doesn't exist yet.

Step 2 (Red!): Run the test, and see the test getting failed, to prove that some changes in the code are required to pass 
the test. Elaboration: _Always_ watch a test fail before you make it pass. If you don't see tests fail,
it's easy to accidentally test code you're not writing and write code that has very little to do with the
tests you're writing.

Step 3 (Green!): Now, write code such that the failing test(s) passes. Elaboration: Don't write bad code just for
fun, but also don't worry too much about slavishly following best practices in everything in this step,
The important thing is to get the tests passing. If you have to commit atrocities to make the tests pass,
go ahead and commit them.

Step 3.5 (Refactor!): Once you see all the tests passing, refactor your atrocities. _Do not_ add any new functionality
to the production code, but rearrange (possibly redesign) it so that it's elegant and pretty. As long as
you don't add any new functionality, you don't have to worry about accidentally screwing something up:
if you do, a test will fail.

Step 4: Once the new code is refactored and beautiful, repeat the process from Step 1.

Maxim from [Justin Searls](https://github.com/searls): "Every line of production code must be
demanded into existence by a failing test."

Note: Any code that doesn't write drives new code is an overhead and it causes the cost of refactoring.
Refer [here](https://github.com/utkarshg6/rust-tests/commit/c8219592c26fc2fcc32b805e3670bd0666c2e235#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R33) for more explanation.

Thank you [Dan](https://github.com/dnwiebe) for explaining me the basics of Test Driven Development, and [Bert](https://github.com/bertllll) for showing me the Hands on experience of writing tests in Rust.
