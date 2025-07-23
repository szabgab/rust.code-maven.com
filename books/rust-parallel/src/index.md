# Rust in Parallel

Every program, every function spends some time executing code in the CPU and some time waiting for the data to arrive. In computational-heavy applications the program might use the GPU instead of the CPU which makes the situation even more complex. For simplicity let's focus on the CPU doing some work for our program vs. our program waiting for some data to arrive from the network. During the latter time our program does not need the CPU.

Let's see a few examples:

## I/O bound application

We are build a crawler. It downloads a web page. Finds links in it then downloads the pages that were linked and goes on like this for 1000 pages.

At first the program uses the CPU to build up the HTTP request and send it to the server. Once the request was sent our program will wait for the answer to arrive.
It might take several seconds. During that time our program waits. Once the content of the page arrives our program starts using the CPU again to extract the links and build up the new requests. It sends out the first request and starts to wait again. When the response arrives stores it and sends out the 2nd request from the first page and starts to wait again.

It has a lot of waiting and little CPU-work in-between.

This is called an I/O bound or I/O intensive or network intensive application,

## CPU bound application

We have a thousand CSV files with with millions of data points in each one. We need to do some computation on each file.

Our program will read the first file into memory. During the reading the CPU does not have much to do, but the reading only takes a few milliseconds (and not seconds as was in the previous example). Then our program does some heavy computation on the file during which the CPU is fully engaged. When it is done it takes the second file. It again spends some little time waiting and then a relatively long time doing the computations.

During most of the time the CPU is working for our program.

This is called a CPU bound, or CPU intensive application.


## Finish sooner

In general we usually want our applications to finish sooner. We prefer to wait 5 minutes for the results instead of 2 hours.

If our application is CPU-bound then we either need a faster CPU or to use more CPUs. Today most computers have many cores that effectively means we have more than one CPU in every computer.
So if we could divide our task to smaller tasks and if we could arrange that some of these task will use one of the cores (CPUs) while other tasks will use another core then we could reduce the overall time needed to do all the work. We might add some extra work, the overhead of managing this, which means we will use more resources of the computer to complete the tasks, but we'll finish them sooner. If this is the situation we have two techniques at our disposal. Creating multiple processes or multiple threads and let the Operating System run them in parallel.
Out of these two techniques using threads will incur a lot less overhead than using processes.


If our application is I/O-bound then we have two possibilities. In one case the network is saturated. That is we use the whole capacity of the cable. This might happen if our bandwidth is small, some other people in our office also have network-intensive applications, or we are downloading movies. In this case we might need to buy more bandwidth, or convince our co-workers to take the day off. However, this is rarely the case. In most situation the network could transfer a lot more data, but it takes time for the data travel all the distance and it takes a lot of time for the other server to process our request and send the data.

In such case it might be beneficial if we could send out several requests at the same time. Then the waiting times of several requests will overlap and we might also be able to do the computation part of one request while the other is still waiting for the response.
In this case we don't even need multiple CPUs to reduce the the time we need to wait as the CPU wasn't doing much in the first place, but having multiple cores (CPUs) might still help.
The techniques at our disposal are: async programming, threads, and processes.






