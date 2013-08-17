#!/bin/sh -ex

cd $(dirname $0)

PATH="`pwd`/../rust-protobuf/src:$PATH"
protoc --proto_path=../protos ../protos/ided.proto --rust_out=.
mv ided.rs ided_proto.rs

# vim: set ts=4 sw=4 et:
