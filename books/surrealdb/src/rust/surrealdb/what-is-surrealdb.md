# What is SurrealDB

* [SurrealDB](https://surrealdb.com/) is a multi-model database written in Rust.

* It has SDK for several languages. Here we'll learn about the Query language(s) of SurrealDB and how to use the database in Rust.

* There are several ways to use the database:
* We can use it embedded or as a separate server.
* Embedded can work with in-memory databse (which is not persistent) or with on-disk backend.
* When using as a separate server we can have one node or multiple nodes.

* One SurrealDB can have multiple **namespaces** and each namespace can have multiple **databases**.
* Each database has separate tables, indices, etc.

* First we need to connect to the database.
* Then we need to authenticate, if necessary.
* then we select the `namespace` and the `database`.

* [SurrealDB for SQL developers](https://surrealdb.com/docs/surrealdb/introduction/sql)
* [SurrealDB for MongoDB developers](https://surrealdb.com/docs/surrealdb/introduction/mongo)


