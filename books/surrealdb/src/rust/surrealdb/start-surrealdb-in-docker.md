# Start SurrealDB in Docker

* Install Docker
* Create a volume to store the data

```
docker volume create my-surreal-db
```

* Start the Docker container using a specific version of SurrealDB image:


```
docker run --detach --restart always --name surrealdb -p 127.0.0.1:8000:8000 --user root -v$(pwd):/external -v my-surreal-db:/database surrealdb/surrealdb:v2.0.4 start --user root --pass root --log trace file://database
```

For the current list of available docker tags check [SurrealDB on the Docker HUB](https://hub.docker.com/r/surrealdb/surrealdb/tags).

* This one will listen on port 8000. You could tell it to listen on some other port. e.g. port 8001:  `-p 8001:8000`.

* Stop the container:

```
docker stop surrealdb
```

* Remove the container

```
docker container rm surrealdb
```

* Remove the volume

```
docker volume remove my-surreal-db
```


