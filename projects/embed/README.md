# embed ([3.3](https://doc.rust-lang.org/book/rust-inside-other-languages.html))

## Setup

Compile with Cargo:

```bash
$ cargo build --release
```

Install Ruby

```bash
$ sudo apt-get install ruby-dev # ubuntu, -dev needed for gem install ffi
```

Install NPM (to run Node example)

```bash
$ sudo apt-get install npm # ubuntu
```

Install the `ffi` gem if necessary:

```bash
$ sudo gem install ffi
```

Install the `ffi` npm package:

```bash
$ npm install ffi
```

## Run

Benchmark the different implementations:

```bash
$ time ruby example.rb
$ time ruby embed.rb
$ time python embed.py
$ time node embed.js
```
