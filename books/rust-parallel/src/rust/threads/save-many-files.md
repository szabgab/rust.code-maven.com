# Save many files (both CPU and IO intensive)

* In this example we demonstrate the speed improvement of threading.
* The program will count the number of prime numbers up to a given number. This part is CPU intensive.
* Then it will create many small files. - This part is IO intensive.

{% embed include file="src/examples/threads/save-many-files/src/main.rs" %}

* Compute primes up to 1 (that is, do almost nothing).  Create 100,000 files. This is mostly IO intensive.
* We can see a 35-40% speed improvement going from no threads to 2 threads, but there is no more speed improvement.

{% embed include file="src/examples/threads/save-many-files/100000_1.out" %}


* Compute primes up to 500 (CPU intensive).  Create 100,000 files. This has both CPU and IO part.
* We can see speed increase by each additional thread, but the improvement diminishes as we add more threads.

{% embed include file="src/examples/threads/save-many-files/100000_500.out" %}


