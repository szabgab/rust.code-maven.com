# Read from a file and return a struct

We have some data file that looks like this:

{% embed include file="src/examples/struct/read-from-file/animals.txt)

We would like to create a function that will read the file and return a vector of structs representing the data.

{% embed include file="src/examples/struct/read-from-file/src/main.rs" %}

In this case we cannot defined the name field to be &str as that will not work
well with the function reading the data from a file.
The read_file will read in the content of the file into owned string.
From that we could create structs that have references in them, but
when we return the structs from the functions the owner of that data will go out of scope
and we'll get a compilation error:
borrowed value does not live long enough



