#!/bin/sh -ex

cd $(dirname $0)

rm -rf src/com/github/stepancheg/rustide/client/proto
protoc -I../protos ../protos/ided.proto --java_out=src

# vim: set ts=4 sw=4 et:
