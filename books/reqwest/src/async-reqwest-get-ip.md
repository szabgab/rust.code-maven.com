# Async reqwest - get IP

In this example we are going to retrieve some data in JSON format. To make this process easy first we are going to retrieve a JSON structure that only has a single key-value pair. The IP address of the client.

[httpbin.org](https://httpbin.org/) provides many API endpoints, one of them is the [/ip](https://httpbin.org/ip) endpoint. Sending a GET HTTP request to this endpoint we get back a JSON structure that looks like this:

```
{
  "origin": "47.131.10.23"
}
```


## Dependencies

In order for this to work we need to add the `json` feature to the `reqwest` crate. We still use `tokio` with the `full` feature as earlier.

```
cargo add reqwest -F json
cargo add tokio -F full
```

## Cargo.toml

{% embed include file="src/examples/reqwest/async-reqwest-get-ip/Cargo.toml" %}

## The Code

{% embed include file="src/examples/reqwest/async-reqwest-get-ip/src/main.rs" %}

We have the helper function `get_url` that allows us to use either the public web site or the one we run locally using Docker.

The first 2 lines are effectively the same as in the previous example, we just retrieve the response as plain text or raw text, if you wish. You can do this to observer the structure of the JSON data.
We don't need this for the real code.

```rust
let raw = reqwest::get(&url).await?.text().await?;
println!("{raw}");
```

Then comes the real code that instead of fetching the content as `text` it fetches it as `json` and immediately converts it to a `HashMap` where both keys and values are `String`s.

The debugging printing of Rust will look quite similar to the original structure except of the trailing comma, and the indentation, I guess.

We can then access the field called `origin`.

That was rather painless because the JSON was very simple.


## The output

```
cargo run
```


```
{
  "origin": "47.131.10.23"
}

{
    "origin": "47.131.10.23",
}
47.131.10.23
```


