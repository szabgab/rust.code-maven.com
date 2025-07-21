# Exercise: character counting


* Given a string count how many time each character appears in the string.

```
Input: "abcax"
Output:
a: 2
b: 1
c: 1
x: 1
```

* First implement a function that does this in a single thread.
* Then create a threaded solution with a shared HashMap where each thread updates the shared HashMap.
* Then create a threaded solution with local HashMaps and then updating the central HashMap at the end of the thread.


