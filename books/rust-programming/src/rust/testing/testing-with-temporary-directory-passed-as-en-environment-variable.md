# Testing with temporary directory passed as an environment variable (test in a single thread RUST_TEST_THREADS)


* config.toml
* RUST_TEST_THREADS
* --test-threads
* env

Because environment variables are per-process and not per thread,
we cannot run the tests in threads (which is the default).

{% embed include file="src/examples/testing/tempfile-with-environment-variable/Cargo.toml" %}

{% embed include file="src/examples/testing/tempfile-with-environment-variable/.cargo/config.toml" %}
{% embed include file="src/examples/testing/tempfile-with-environment-variable/src/main.rs" %}


