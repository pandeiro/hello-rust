# hello-http

A simple HTTP server written using
[hyper](https://github.com/hyperium/hyper), a third-party HTTP library
for Rust.

### Building and Running

Get the prerequisite OpenSSL libs and headers
([source](https://github.com/hyperium/hyper/issues/709#issuecomment-172326256)):

```bash
$ sudo apt-get install libssl-dev
```

Run with Cargo:

```bash
$ cargo run
```

Or build and then run:

```bash
$ cargo build
$ ./target/debug/hello_http
```

### Testing

Test the server:

```bash
$ curl http://127.0.0.1:3000  # => Hello World!
```
