# httpbin


[httpbin](https://httpbin.org/) is a service to test HTTP client implementations. We are going to use it extensively to see examples of code.

The service is quite reliable, but it isn't always available, but luckily the developer provides it as a [Docker](https://docker.com/) container as well.

So if you have Docker installed on your computer you can run the service locally.

## Run httpbin locally

Start as:

```
docker run --rm -p 80:80 --name httpbin kennethreitz/httpbin
```

Then run the code:

```
cargo run localhost
```

To stop the Docker container open another terminal and execute

```
docker container stop -t0 httpbin
```


