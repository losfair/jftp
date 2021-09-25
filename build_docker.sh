#!/bin/bash

set -e

cargo build --release --target x86_64-unknown-linux-musl
tmpdir="$(mktemp -d --suffix jftpbuild)"
cp ./target/x86_64-unknown-linux-musl/release/jftp "$tmpdir/"
cp ./Dockerfile "$tmpdir/"
docker build -t "losfair/jftp" "$tmpdir"
rm -rf "$tmpdir"
echo "Built JFTP docker image."
