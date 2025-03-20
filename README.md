# rebox - remaking busybox with rust

this is a project to remake busybox with rust. the goal is to make a small, fast, and featureful shell with a (relatively, we are at the mercy of the rust compiler here) small binary size.

# work in progress

this project is still in the early stages of development. it is not yet feature complete, and is not yet ready for general use. if you are interested in contributing, please see the [CONTRIBUTING.md](CONTRIBUTING.md) file.

# building

```terminal
$ cargo build --release
$ cp target/release/box ./cat # or any other command
$ ./cat /etc/passwd
root:x:0:0:root:/root:/bin/bash
$ cp ./cat ./echo
$ ./echo hello, world!
hello, world!
$ cp ./echo ./box
$ ./box echo hello, world!
hello, world!
$ ./box cat /etc/passwd
root:x:0:0:root:/root:/bin/bash
```