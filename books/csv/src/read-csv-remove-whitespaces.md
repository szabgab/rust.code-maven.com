# Read CSV remove (trim) whitespaces from around the values

* serde
* alias
* Deserialize
* ReaderBuilder

* Sometimes we have a CSV file where the data is aligned nicely using spaces to pad the values. (Most likely a manually maintained CSV file).
* We can tell the CSV reader to trim down does whitespaces.

* In this example we also used the `alias` attribute of `serde` to map the real titles in the CSV file to the fieldnames of the struct that can use a much more limited set of characters.

{% embed include file="src/examples/csv/csv-trim-all-the-whitespaces/planets.csv" %}
{% embed include file="src/examples/csv/csv-trim-all-the-whitespaces/src/main.rs" %}
{% embed include file="src/examples/csv/csv-trim-all-the-whitespaces/Cargo.toml" %}
{% embed include file="src/examples/csv/csv-trim-all-the-whitespaces/out.out" %}


