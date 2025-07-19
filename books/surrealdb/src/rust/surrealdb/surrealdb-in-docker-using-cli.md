# SurrealDB in Docker using the CLI

* TODO: types of data?
* TODO: schema?
* TODO: index


* We would like to try SurrealDB, but we rather use Docker instead of installing SurrealDB on our computer.
* At first we'll use the SurrealDB command line with an in-memory (not persistent) database.

We could use the `latest` tag, but to make sure total reproducability we prefer to pick the [latest tag](https://hub.docker.com/r/surrealdb/surrealdb/tags) in the  2.* series.

```
$ docker run -it --rm  --user root surrealdb/surrealdb:v2.0.4 sql --endpoint memory --ns ns --db db --pretty
```

* Using `ns` as our namespace and `db` as our database so they won't take much real estate in the prompt. Feel free to use anything else.


* Create the first entry in the new `planet` table with an automatically generated ID.

```
ns/db> CREATE planet SET name = 'Earth';
-- Query 1 (execution time: 906.201µs)
[
	{
		id: planet:l0tpjh7uykux7sr5johy,
		name: 'Earth'
	}
]
```

* Create another entry and set the ID manually.

```
ns/db> CREATE planet:4 SET name = 'Mars';
-- Query 1 (execution time: 315.322µs)
[
	{
		id: planet:4,
		name: 'Mars'
	}
]

```


```
ns/db> CREATE planet SET name = 'Mercury', distance = 0.7;
-- Query 1 (execution time: 544.853µs)
[
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]

ns/db> select * from planet;
-- Query 1 (execution time: 176.03µs)
[
	{
		id: planet:4,
		name: 'Mars'
	},
	{
		id: planet:6xg3xbsnzbkyolssmtb6,
		name: 'Earth'
	},
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]
```

```
ns/db> INFO for db;
-- Query 1 (execution time: 183.976µs)
{
	accesses: {},
	analyzers: {},
	configs: {},
	functions: {},
	models: {},
	params: {},
	tables: {
		person: 'DEFINE TABLE person TYPE ANY SCHEMALESS PERMISSIONS NONE',
		planet: 'DEFINE TABLE planet TYPE ANY SCHEMALESS PERMISSIONS NONE'
	},
	users: {}
}
```

* Select all the items from more than one table

```
ns/db> SELECT * from person, planet
-- Query 1 (execution time: 254.849µs)
[
	{
		home: planet:4,
		id: person:ea6qfpbi8mu9prnn8nfx,
		name: 'Elon Musk'
	},
	{
		home: planet:6xg3xbsnzbkyolssmtb6,
		id: person:f2c3w5699hx64q1guta6,
		name: 'Gabor'
	},
	{
		id: planet:4,
		name: 'Mars'
	},
	{
		id: planet:6xg3xbsnzbkyolssmtb6,
		name: 'Earth'
	},
	{
		distance: 0.7f,
		id: planet:hxkrucoezhwmtayok509,
		name: 'Mercury'
	}
]

```


