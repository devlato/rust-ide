# rust-ide

Integrated development environment for [Rust](http://www.rust-lang.org/).

Work in progress. Not even a prototype yet.

## Architecture

* librustc and libsyntax taken from rust
* `ided` process, written in rust, linked with librustc
* library written in java, that spawns `ided` command,
  and talks to it through pipe
* plugins to your favorite IDE that uses that java library.
  First will likely be IDE from [JetBrains](http://www.jetbrains.com/idea/).

## Project structure

* rust/ — fixed version of rust
* ided/ — headless IDE
* protos/ — protobuf files describing protocol of commuincation with `ided`
* [rust-protobuf](https://github.com/stepancheg/rust-protobuf/)/
  — protobuf implementation in and for rust
* ided-client-java/ — java API to `ided`

## Rusty Cage

There is an IDE for rust —
[Rusty Cage](https://github.com/reidarsollid/RustyCage).
I think it is dead end, because it does not utilize
rust compiler for code analysis. I hope to merge
with that project one day.


## Stay tuned!

Contact me if you want to join! <stepan.koltsov@gmail.com>
