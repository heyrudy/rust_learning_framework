# What about cargo there ?

### **Cargo**

Cargo is Rust’s built-in package manager and the build system. It can be used to,

* Create a new project: `cargo new`
* Create a new project in an existing directory: `cargo init`
* Create a new project from template: `cargo generate`, this new operation require installing the
                                       **cargo-generate** binary by running `cargo install cargo-generate`
* Add dependency to the project: `cargo add`, this one require installing the
                                 **cargo-edit** binary by running `cargo install cargo-edit`
* Format the project source code: `cargo fmt`
* Build the project: `cargo build`
* Build the project on change: `cargo watch` this one require installing the
                               **cargo-watch** binary by running `cargo install cargo-watch`
* Clean the **`target`** folder: `cargo clean`
* Run the project: `cargo run`
* Update project dependencies: `cargo update`
* Run tests (**`doctests`** or files under **`test/`** folder): `cargo test`
* Run benchmarks: `cargo bench`
* Generate the project documentation via rustdoc: `cargo doc`
* Analyze the project to see it has any errors, without building it: `cargo check`
* Login to crates.io with the API token: `cargo login`
* Make the local crate uploadable to crates.io: `cargo package`
* Upload the crate to crates.io: `cargo publish`
* Install a Rust binary: `cargo install`
* Uninstall a Rust binary: `cargo uninstall`


### **...Clippy**
**`clippy`** is a "linting" tool for Rust, similar to standard's style lints.  
`cargo fmt` takes care of formatting.  
`cargo build` takes care of correctness.  
 But `clippy` is in charge of helping you write **"idiomatic"** Rust.

This doesn't mean every lint in Clippy is perfect. But when you're getting started it can be suuuper helpful to run!