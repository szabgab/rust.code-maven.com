# Testing


Writing unit, integration, acceptance, regression, performance, etc. tests in Rust.

Writing unit-, integration-, etc. tests should be an integral part of the development work, but in my experience in many organizations it is more like an afterthought. Sometimes relegated to a separate team or separate department.

Some people put a lot of emphasize on the separation between unit-, integration-, and acceptance testing. They are all part of the larger group called **functional testing**. I think it is primarily a question of scope. They all have the same equation:

**Fixture + Input = Expected Output + Bugs**

In this series of articles we are going to cover how one could write tests in Rust.

* In the series about the [Rocket web development framework](https://rust.code-maven.com/rocket) each example in each article comes with its tests.

* [Show standard output and standard error in tests](./show-output-in-tests.md)
* [Test command line applications](./test-command-line-application.md)


* There is a nice collection of [types of testing](https://www.geeksforgeeks.org/types-software-testing/).
