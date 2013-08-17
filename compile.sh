#!/bin/sh -ex

cd $(dirname $0)

(
    cd rust
    make
) || exit 1

(
    PATH="`pwd`/rust/x86_64-apple-darwin/stage2/bin:$PATH"
    cd rust-protobuf/src
    ./rebuild.sh
) || exit 1

PATH="`pwd`/rust/x86_64-apple-darwin/stage2/bin:$PATH"

ided/compile.sh

# vim: set ts=4 sw=4 et:
