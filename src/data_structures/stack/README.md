# Doc

# Test

## Add test

1. add 'cargo.toml':

    [package]
    name = "stack"
    version = "0.2.0"
    authors = ["HuawenYu"]

2. add lib list 'lib.rs', which is the file name's list:

    pub mod stack_list;
    pub mod stack_vec;

## Run test

```sh
cargo test
```
