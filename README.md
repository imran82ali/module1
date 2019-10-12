# Sample Crate for Student Learning 
This is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`module1 = "0.1.3"`

your cargo.toml file should look like this:
```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["imran82ali <code.imranali@gmail.com>"]
edition = "2018"

[dependencies]
module1 = "0.1.3"
```

In `src/main.rs` you can use like this:

```
use module1;
fn main() {
    println!("Hello, world!");
    module1::hello();
}
```
following will also work:
```
use module1::hello;
fn main() {
    println!("Hello, world!");
    hello();
    }
```

now `cargo run` for results
