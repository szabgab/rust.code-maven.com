# Exercise: Download many files in threads

In the chapter about http we had an example for a blocking http client and we had an example downloading all the pages of the Rust Maven site using Tokio and async calls.

Implement the "download many files" application using threads and blocking the http client. Make the main thread collect the sizes of the downloaded pages.


