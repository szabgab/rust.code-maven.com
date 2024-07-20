# http
{id: http}

## http-client with reqwest
{id: http-client}

* [reqwest](https://crates.io/crates/reqwest)
* [tokio](https://crates.io/crates/tokio)

![](examples/reqwest/http-client/Cargo.toml)
![](examples/reqwest/http-client/src/main.rs)


## Simple blocking http client
{id: simple-blocking-http-clients}

```
cargo add reqwest --features blocking
```

![](examples/reqwest/simple-http-client/Cargo.toml)
![](examples/reqwest/simple-http-client/src/main.rs)

## Download many URLs in parallel (async)
{id: download-many-urls-async}
{i: reqwest}
{i: async}
{i: tokio}

![](examples/reqwest/download-rust-maven/src/main.rs)
![](examples/reqwest/download-rust-maven/Cargo.toml)

Based on [this article](https://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust)
