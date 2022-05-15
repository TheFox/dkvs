# dkvs

Distributed Key-Value Storage written in Rust.

## Project Outlines

The project outlines as described in my blog post about [Open Source Software Collaboration](https://blog.fox21.at/2019/02/21/open-source-software-collaboration.html).

- The main purpose of this software is to provide a cluster of servers, which all store the same data as key-value.
- This list is open. Feel free to request features.

## Tasks

```rust
// Tasks POC
let mut test3: RefCell<u64> = RefCell::new(0);
manager.add_task("Test1".into(), Duration::new(5, 0), || {
    *test3.borrow_mut() += 10000;
});

loop {
    *test3.borrow_mut() += 1;
    println!("-> run test3: {:?}", test3);
}
```
