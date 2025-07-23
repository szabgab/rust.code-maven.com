# A literal string cannot be changed

If we go back to the variable that was holding a literal string, you can see that while we could replace the content of the variable
there are no methods allowing us to change the string. Eg. `push_str` does not exist for literal strings.

{% embed include file="src/examples/variables/change-literal-string/src/main.rs" %}

Compilation error:

{% embed include file="src/examples/variables/change-literal-string/out.out" %}


---

* push_str

