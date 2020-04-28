# What about crate there ?

### **Crate**

A **crate** is a package, which can be shared via crates.io.  
A **crate** can produce `an executable` or `a library`. In other words, it can be `a binary crate` or `a library crate`.

1. `cargo new crate_name --bin` or `cargo new crate_name`: Produces `an executable`
2. `cargo new crate_name --lib`: Produces `a library`

`cargo new crate_name --bin` generates,

```console
├── Cargo.toml
└── src
    └── main.rs
```

`cargo new crate_name --lib` generates,

```console
├── Cargo.toml
└── src
    └── lib.rs
```

* **Cargo.toml**(capital c) is the configuration file which contains all of the metadata that Cargo needs to compile your project.
* **`src/`** folder is the place to store the source code.
* Each **crate** has an implicit **crate root**/**entry point**.
  + **main.rs** is the crate root for a binary crate
  + **lib.rs** is the crate root for a library crate