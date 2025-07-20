# Parametrization of tests

I could not find parametrization as exists in pytest, but we can fake it by creating a function that accepts
the parameters and then creating many test functions calling that function.

{% embed include file="src/examples/testing/fake-parametrize/src/lib.rs" %}


