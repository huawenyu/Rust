# Doc
Read the pretty version at https://rust-unofficial.github.io/too-many-lists/.

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
