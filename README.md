# My Rust lang hacks


## Tutorial desc
SW written here is based on the following resources:
* Official documentation:
    https://doc.rust-lang.org/stable/book/
    https://doc.rust-lang.org/rust-by-example
* Rustlings: https://github.com/rust-lang/rustlings
* Rust for cpp devs: https://github.com/nrc/r4cppp
* Book: ["Practical Rust Projects"](https://www.amazon.com/Practical-Rust-Projects-Computing-Applications/dp/1484255984)


### Rust env setup
Follow the instructions from: https://www.rust-lang.org/learn/get-started
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Create and build a new project
Look here: `https://doc.rust-lang.org/cargo/getting-started/first-steps.html`
To start a new package with Cargo, use cargo new:

```
$ cargo new hello-rust
```
Cargo defaults to --bin to make a binary program. To make a library, we'd pass --lib.

Let’s check out what Cargo has generated for us:
```
$ cd hello-rust
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```
This is all we need to get started. First, let’s check out Cargo.toml:

```
[package]
name = "hello-rust"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
This is called a manifest, and it contains all of the metadata that Cargo needs to compile your package.
```

Here’s what’s in src/main.rs:

```
fn main() {
    println!("Hello, world!");
}
```
Cargo generated a “hello world” for us. Let’s compile it:

```
$ cargo build
   Compiling hello-rust v0.1.0 (file:///path/to/package/hello-rust)
```

And then run it:
```
$ ./target/debug/hello-rust
Hello, world!
```

We can also use cargo run to compile and then run it, all in one step:
```
$ cargo run
```
     Fresh hello-rust v0.1.0 (file:///path/to/package/hello-rust)
   Running `target/hello-rust`
Hello, world!


