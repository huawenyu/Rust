# Doc
[The book of how-to implement a better list](https://rust-unofficial.github.io/too-many-lists/index.html)
And [A pretty version](https://rust-unofficial.github.io/too-many-lists/)

# Run test

```sh
cargo test
cargo test run_this_special_test_fn
```

# Building

Building requires mdbook, which can be installed from crates.io:

```sh
cargo install mdbook
```

Assuming you've placed the install directory `~/.cargo/bin` into your system PATH, then run from the root of your local copy:

```sh
mdbook build
```

## std::mem::replace

It literally replaces the original value in dest with the new value in src, but does not deallocate the original value itself (unlike *dest = src;) and returns it instead. It does perform the memory copy, but it wouldn't call any destructor just like swap. In fact, it is a swap in disguise if you look at its implementation.

So why do we need that? One major use case is as follows: We want to consume some self field and give that to another self method which wouldn't read or alter that field but would alter others. Unfortunately the borrow checker is not that precise, so it would result in an error. This is quite annoying since a separate function wouldn't solve this issue; the only workaround is to move and replace the referenced self field before calling any mutable method:

let dummy_val = ...;
let orig_val = std::mem::replace(&mut self.to_be_consumed, dummy_val);
// now `self.to_be_consumed` equals to `dummy_val`, and we have
// `orig_val` detached from any lifetime associated to `self`.
// we can now pass `orig_val` to any function we want, including:
self.mutable_method(orig_val);
