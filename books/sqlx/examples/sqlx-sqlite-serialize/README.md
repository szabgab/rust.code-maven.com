# Backup and restore

Based on the [sqlite serialize](https://github.com/launchbadge/sqlx/tree/main/examples/sqlite/serialize) example.

Show how we can serialized (and save to disk) the content of the database
and then how we can restore the data from the serialized dump.



## Dependencies

Apparently this feature is not in the release version yet.

```
cargo add --git https://github.com/launchbadge/sqlx sqlx -F sqlite-deserialize -F sqlite -F runtime-tokio
```
