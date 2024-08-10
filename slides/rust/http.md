# http
{id: http}

## httpbin
{id: httpbin}

* [httpbin](https://httpbin.org/) a service to test http client implementations.


## Simple blocking http client with reqwest
{id: simple-blocking-http-clients}
{i: reqwest}
{i: blocking}
{i: get}
{i: status}

```
cargo add reqwest --features blocking
```

![](examples/reqwest/simple-http-client/Cargo.toml)
![](examples/reqwest/simple-http-client/src/main.rs)


## http-client async with reqwest
{id: http-client}
{i: reqwest}
{i: async}
{i: tokio}

* [reqwest](https://crates.io/crates/reqwest)
* [tokio](https://crates.io/crates/tokio)


```
apt-get install pkg-config
apt-get install libssl-dev
```


![](examples/reqwest/http-client/Cargo.toml)
![](examples/reqwest/http-client/src/main.rs)


## Download many URLs in parallel (async)
{id: download-many-urls-async}
{i: reqwest}
{i: async}
{i: tokio}

![](examples/reqwest/download-rust-maven/src/main.rs)
![](examples/reqwest/download-rust-maven/Cargo.toml)

Based on [this article](https://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust)
