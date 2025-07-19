# Simple TODO list with Rocket and SurrealDB


* TODO: why is the item.id.id shown as [object] in the web page while printing to the log shows the ID only.


Setup server:


```
docker volume create my-surreal-db
docker run --detach --restart always --name surrealdb -p 127.0.0.1:8000:8000 --user root -v my-surreal-db:/database surrealdb/surrealdb:v2.0.1 start --user root --pass root --log trace file://database
```

Dependencies

{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/Cargo.toml" %}

{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/Rocket.toml" %}

{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/src/main.rs" %}
{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/src/db.rs" %}

{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/templates/index.html.tera" %}

{% embed include file="src/examples/rocket/simple-todo-with-surrealdb/templates/item.html.tera" %}


