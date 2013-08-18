#!/bin/sh -ex

cd $(dirname $0)

. setenv.sh

./gen-proto.sh
rustc -L../rust-protobuf/src/lib ./ided.rc

# vim: set ts=4 sw=4 et:
