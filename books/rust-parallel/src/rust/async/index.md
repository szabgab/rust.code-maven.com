# async

* Especially for IO intensive applications


* Reading from a file, but many disks actually cannot do parallel reading or writing so there is not much gain. Unless we have several disks in the computer or unless the disks are mounted from some other devices via `nfs` or `Samba`.

* Network client operations such as downloading pages from the Internet or accessing APIs throught the network.

* Network server operations where our system needs to handle multiple perations that involve network operations.

* Accessing databases (via the network), which in reality is just accessing APIs over some protocol. (Which is probably not http.)

