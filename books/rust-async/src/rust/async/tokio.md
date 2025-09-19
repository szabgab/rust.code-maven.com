# tokio

* [tokio crate](https://crates.io/crates/tokio)
* [tokio web site](https://tokio.rs/)


Before we delve into the learning of async and tokio, let's see the results of two examples.

One of them is an HTTP client to download many pages.

The other is an HTTP server to serve many pages.

For both we have a sync and async version.

* If we run the clients downloading many external pages we can observe the speed improvement from sync to async. In this case we don't really care how those servers handle the requests.

* If we run the servers we either need our sync and async clients to demonstrate, or we can launch some other program to observer the results.






