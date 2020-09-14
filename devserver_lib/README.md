# devserver_lib
devserver_lib does (nearly) the minimum necessary to serve a static folder over https://localhost:8080.

**DO NOT USE DEVSERVER_LIB IN PRODUCTION**

`devserver_lib` should only be used for locally hosting files on a trusted network. 

`devserver_lib` does not properly handle the attacks robust servers must withstand on an open network.

## usage
```rust
extern crate devserver_lib;

fn main() 
{
  devserver_lib::run(&"localhost", 8080, "", /*Auto-reload:*/ true ); // Runs forever serving the current folder on http://localhost:8080
}
```

## dependencies
[rust-native-tls](https://github.com/sfackler/rust-native-tls)

Dependencies only for the reload feature:
[notify](https://github.com/notify-rs/notify)
[sha-1](https://github.com/RustCrypto/hashes)
[base64](https://github.com/marshallpierce/rust-base64)

## Resources to learn from
https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

http://concisecoder.io/2019/05/11/creating-a-static-http-server-with-rust-part-1/
