# What about this hello world flavor 1 ?

### First thing first, you are inside a binary crate here

I divide the code structure of our binary crate around these two files in `src/`folder:

* **[main.rs](https://github.com/Turing00/rust_learning_framework/blob/master/examples/hello_world_flavor1/src/main.rs)** for the entry point of our binary crate
* **[lib.rs](https://github.com/Turing00/rust_learning_framework/blob/master/examples/hello_world_flavor1/src/lib.rs)** for a support module of our binary crate

This is what **main.rs** almost looks like:
```rust
...
fn main() {
	println!("{}", hello_world_flavor1::greeting());
}
```

And this is what **lib.rs** almost looks like:
```rust
pub fn greeting() -> String {
    "Hello, world!".to_string()
}
...
```

### A quick notice here:  
`fn` means function.  
The `main` function is the beginning of every Rust program.  
`println!` prints text to the console and its `!` indicates that it’s a macro rather than a function.  
`pub` is basically a **reserved keyword** to expose things to the outside world.  
`->` this arrow in the function signature means that the function return a value.