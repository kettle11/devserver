# devserver_lib
This tiny Rust library does (nearly) the minimum necessary to serve a static folder over https://localhost:8000.

This is not an example of elegance, security, functionality, or good Rust code. But it is simple. And it does serve a folder over https (or http) in under 100 lines of actual logic with only the standard library and one small external dependency.

This may be useful as an example, or adapted for cases requiring an extremely simple development server.

## usage
```rust
extern crate devserver_lib;

fn main() 
{
  devserver_lib::run("localhost:8000", false); // Runs forever serving the current folder on http://localhost:8000
}
```

## dependencies
[rust-native-tls](https://github.com/sfackler/rust-native-tls)

## Resources to learn from
https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

http://concisecoder.io/2019/05/11/creating-a-static-http-server-with-rust-part-1/
