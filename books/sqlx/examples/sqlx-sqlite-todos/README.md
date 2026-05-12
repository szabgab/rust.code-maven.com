# TODOs Example

Based on the one in the [source code](https://github.com/launchbadge/sqlx/tree/main/examples/sqlite/todos)


## Setup

0. Install `sqlx-cli`.

1. Declare the database URL

    ```
    export DATABASE_URL="sqlite:todos.db"
    ```

2. Create the database.

    ```
    $ sqlx db create
    ```

3. Run sql migrations

    ```
    $ sqlx migrate run
    ```

## Usage

Add a todo

```
cargo run -- add "todo description"
```

Complete a todo.

```
cargo run -- done <todo id>
```

List all todos

```
cargo run
```
