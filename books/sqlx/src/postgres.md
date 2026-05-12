# Postgres


Start the Postgres server in a Docker container


```shell
$ docker run -d -e POSTGRES_PASSWORD=password -p 5432:5432 --name postgres postgres:latest
```


At the end shut it down:

```shell
$ docker container stop postgers
```
