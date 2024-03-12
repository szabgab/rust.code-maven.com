---
title: Strings and memory allocation in Rust
timestamp: 2024-03-12T15:30:01
author: szabgab
published: true
description: See how Rust enlarges memory allocations and how it moves the data when even more space is required.
tags:
    - String
    - push_str
    - replace_range
    - :p
    - len
    - capacity
    - as_ptr
todo:
    - use push_str to make the example more simple to understand
---

In this example we'll explore the memory allocation of Rust as we change the content of a `String`.

The `:p` placeholder in `println!("{:p}", &text)` will print the hexadecimal address of the variable.
The variable itself contains some meta-data about the content of the variable. e.g. Where it is located, how long it is and how much total space is allocated for growth
before new memory needs to be allocated and the the content has to be moved.


The `as_ptr` method returns the address of the actual content of the variable what I think some people might call "payload".

In the code after printing the two addresses we also print the length of the string, the total capacity of the variable and the actual content of the variable.
In `{:<15?}` the 15 means to pad the printed value up to 15 places. I use  it to make sure the columns are aligned nicely. the `<` sign means that the content needs to be
left-aligned.


* At first we create an empty string and put it in a mutable variable. This creates the variable, but does not allocate any space for the content as there is no content yet. That's why ptr os `0x1`.
* The append some text using the `push_str` method. the string itself is 10 bytes long and rust allocates exactly 10 bytes. At this point the we already have the address of the data: `0x5de087754ba0`.
* Then we user `replace_range` to replace some of the content by other content of the exact same length. There is no need for extra capacity and the length of the string does not change either.
* Then we create a new variable with some text and print it. The point of this little exercise is to make sure the memory after our variable is used. As you can see the address of the content of this variable is `0x5de087754bc0`. It is 32 bytes more than the address of the data in our `text` variable.
* At this point we used the `replace_range` method again, but this time we replaced the 3-bytes long slice by a 4-bytes long string. This means the rest of the string had to be moved one byte higher. It also meant the string is now 11 bytes long. However Rust allocated 9 additional bytes to a total of 20 bytes anticipating further growth. This happened while the address of the data remained `0x5de087754ba0`. This could have happened because between `0x5de087754ba0` and `0x5de087754bc0` there was some extra unused space. I am not sure why was that unused space left there, but I am not complaining...
* Then I enlarged the string a little-bit more replacing 3 bytes by 6. This changed the length of the string to 14, but did not change the capacity as the existing capacity (20) was enough.
* Finally we added even more text. This made the string 28 characters long. As an optimization Rust allocated some ore space for additional growth to have a total capacity of 40 bytes. However there were only 32 bytes before the data of the `temp` variable and thus Rust had to allocate new space and move all the data to the new location. The address of the variable is still the same `0x7ffdcc572770` as we had at the beginning. However the address of the data is now `0x5de087754be0`.


{% include file="examples/memory-and-strings/src/main.rs" %}


```
p: 0x7ffdcc572770 ptr: 0x1             len:  0, capacity:  0 ''
p: 0x7ffdcc572770 ptr: 0x5de087754ba0  len: 10, capacity: 10 'Hello foo!'
p: 0x7ffdcc572770 ptr: 0x5de087754ba0  len: 10, capacity: 10 'Hello bar!'
The black cat
p: 0x7ffdcc573260 ptr: 0x5de087754bc0  len: 13, capacity: 13 'The black cat'
p: 0x7ffdcc572770 ptr: 0x5de087754ba0  len: 11, capacity: 20 'Hello qqrq!'
p: 0x7ffdcc572770 ptr: 0x5de087754ba0  len: 14, capacity: 20 'Hello 123456q!'
p: 0x7ffdcc572770 ptr: 0x5de087754be0  len: 28, capacity: 40 'Hello 12345678901234567890q!'
```


## Notes

I hope this helps you understand a bit better how the memory is handled by rust.

Of course allocating new memory and moving the string is very time-consuming, so if you know up front how long your string will become
you can pre-allocate enough space. But that's another story.



