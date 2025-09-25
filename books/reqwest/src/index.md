# HTTP client in Rust

There are a number of Crates that can help us build HTTP requests. The most popular seems to be [reqwest](https://crates.io/crates/reqwest). We are going to take a look at it.

Herbert Wolverson commented that reqwest spawns a tokio thread even when working in blocking mode which is quite expensieve and hes suggested [ureq](https://crates.io/crates/ureq). We'll take a look at that as well.


We'll use [httpbin](https://httpbin.org/) for checking examples.

