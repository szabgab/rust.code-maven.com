# Lifetime
{id: lifetime}


## Lifetime elision rules
{id: lifetime-elision-rules}

```
rustc --explain E0106
```

## Function receiving and returning one reference
{id: function-receiving-and-returning-one-reference}

* the elision rules apply and thus we don't need lifetime specifiers

![](examples/lifetime/function-one-param/src/main.rs)
![](examples/lifetime/function-one-param/out.out)


## Function receiving two and returning one reference
{id: function-receiving-two-and-returning-one-reference}

![](examples/lifetime/function-two-params/src/main.rs)
![](examples/lifetime/function-two-params/out.out)


## Exercise: longest string
{id: exercise-lifetime-longest-string}

* Implement a function that received 3 strings and returns the longest string

```
fn longest(a: &str, b: &str, c: &str) -> &str
```

* You will probably need some lifetime annotations.

## Exercise: lifetime
{id: exercise-lifetime-longer-or-static}

* Implement a function that receives 2 strings and returns one string.
* If the first string is longer than the second string return the first string.
* Otherwise return the string "nope".


