# Accessing characters in a string


```rust
text[3]
```

* Is this the 4th byte or the 4th unicode character?
* The former would be garbage if we have anything else besides ASCII in the string.
* The latter would mean accessing elements is not a O(1) operation as we would have to iterate over all the previous characters.


