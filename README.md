# Rust Tests

This project contains tests in Rust, in a beginner-friendly manner.

To run tests, you can run the command:

```zsh
cargo test
```
This subcommand has a lot of possible arguments that you can use to change the setup of your test, see official documentation for more information
[cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

Here's a [guide](https://github.com/utkarshg6/helpful/blob/master/Rust.md#rust-tests) that can be used as a reference for writing tests in Rust.

## A Step by Step guide of the Test Driven Development:

Step 1: **Don't** write any source code, write tests first. Elaboration: It's okay to frame out production data
structures and code patterns without tests; but they should contain no valid behavioral code. Use macros
like `unimplemented!()` or `todo!()` (the latter is prefered, [see this](#unimplemented-vs-todo)) to hold places for code that doesn't exist yet.

Step 2 (Red!): Run the test, and see the test getting failed, to prove that some changes in the code are required to pass
the test. Elaboration: _Always_ watch a test fail before you make it pass. If you don't see tests fail,
it's easy to accidentally test code you're not writing and write code that has very little to do with the
tests you're writing.

Step 3 (Green!): Now, write code such that the failing test(s) passes. Elaboration: Don't write bad code just for
fun, but also don't worry too much about slavishly following best practices in everything in this step,
The important thing is to get the tests passing. If you have to commit atrocities to make the tests pass,
go ahead and commit them.

Step 3.5 (Refactor!): Once you see all the tests passing, refactor your atrocities. _Do not_ add any new functionality
to the production code, but rearrange (possibly redesign) it to be more elegant and pretty. As long as
you don't add any new functionality, you don't have to worry about accidentally screwing something up:
if you break the existing one, a test will fail.

Step 4: Once the new code is refactored and beautiful, repeat the process from Step 1.

Maxim from [Justin Searls](https://github.com/searls): "Every line of production code must be
demanded into existence by a failing test."

Note: Any code that doesn't drive new code is an overhead and it causes the cost of refactoring.
Refer [here](https://github.com/utkarshg6/rust-tests/commit/c8219592c26fc2fcc32b805e3670bd0666c2e235#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R33) for more explanation.

### `unimplemented!()` vs `todo!()`
At least for us working in IntelliJ, `todo!()` has the nice part of not shadowing and deactivating the piece of code taking place after it unlike with `unimplemented!()` which does. That causes that you cannot get the maximum potential out of the tools provided with the editor's Rust plugin which is there to make your work easier, though.  

## Test Coverage

### What is Test Coverage?

In computer science, test coverage is a measure (in percent) of the degree to which the source code of a program is executed when a particular test suite is run. Refer [Wikipedia](https://en.wikipedia.org/wiki/Code_coverage).

### Why Test Coverage is not a good metric to judge proper testing by a particular test suite?

#### False positives and false negatives

Whenever we consider a hypothesis, the error cases are categorized with the following terms:

- **False Positive** _[Type 1 Error]_: We expected something to be _positive_ but in reality that is _negative_.
- **False Negative** _[Type 2 Error]_: We expected something to be _negative_ but in reality that is _positive_.

Note: False Negatives (or Type 2 Errors) are considered to be far riskier than False Positives. For Example, If you receive a piece of news that there is **not** going to be an earthquake but in reality it will happen (False Negative) is far riskier than If you receive a piece of news that there was going to be an earthquake but it didn't happen (False Positive).

#### Relation with Test Coverage

* Using Test Coverage as the main metric increases the chances of False Negatives (riskier Type 2 Errors).
* When the Test Coverage increases we start to believe that more lines covered means also fewer bugs, which keeps us far away from the truth as we deprioritized the logic testing over code coverage.
* Thus, as we incline to believe that more lines covered means fewer bugs it makes us more prone to False Negatives.
* Test quality is more than quantity
  * Given e.g. 20% code coverage, the only benefit Test Coverage serves is to guide us to an awareness that we've definitely written too few tests. Obviously not enough to cover our code, because about 80% is left. But if we have 80% Test Coverage that does not necessarily mean that we've created reliable tests proving considerably valuable or useful things.
  * It's possible that our tests are pointless; they don't provide exact assertions on results we would've expected at certain places in our code. The percentage only means how much code was run during the process, nothing about the quality of the produced results. Nothing about the fact whether we tested what's going on in important parts of the code and whether it fits to the expected things and thus the initial assignment.
  * Ironically, the tests might not have any assertions in them at all and still they may run nicely and take a portion in an increase of the total Test Coverage. 
  * The results we want to see should be expected in response to the initial test conditions we set up for each test. If we can find these assertable (understand timely immutable, constant or if not, then computable) values and we also get them eventually right as our test results, both sides of the assertion to be a match, with existence of the proper target functionality behind that, we can be confident that we have a much stronger coverage than we would've had when using an ordinary Test Coverage tool of some choice. That's when the implementer's resigned on or missed the idea of precise assertions with all parts of the functionality we've been driving in.  
* Thus, we should always use Test-Driven Development (unless we are too lazy, but then nobody can help you).

## Thanks

Thank you [Dan](https://github.com/dnwiebe) for explaining me the basics of Test Driven Development, and [Bert](https://github.com/bertllll) for showing me the Hands on experience of writing tests in Rust.
