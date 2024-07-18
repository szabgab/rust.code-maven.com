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

