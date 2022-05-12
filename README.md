# expr-fsm-1

A first experiment on creating a Finite-State Machine in rust.
This time using &dyn State rather than box as in expr-traits.
But same result:

```
wink@3900x 22-05-12T04:53:12.411Z:~/prgs/rust/myrepos/expr-fsm-1 (dyn-state)
$ cargo run
   Compiling expr-fsm-1 v0.2.0 (/home/wink/prgs/rust/myrepos/expr-fsm-1)
error[E0499]: cannot borrow `*self` as mutable more than once at a time
  --> src/main.rs:10:20
   |
9  |         let cs = self.cur_state();
   |                  ---------------- first mutable borrow occurs here
10 |         cs.process(self, msg);
   |            ------- ^^^^ second mutable borrow occurs here
   |            |
   |            first borrow later used by call

For more information about this error, try `rustc --explain E0499`.
error: could not compile `expr-fsm-1` due to previous error
wink@3900x 22-05-12T04:54:31.572Z:~/prgs/rust/myrepos/expr-fsm-1 (dyn-state)
```

## Building and running

Fails as above :(

## Test

TODO

## Benchmark

TODO

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
