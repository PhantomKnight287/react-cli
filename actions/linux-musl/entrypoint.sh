#!/usr/bin/env bash
mkdir .cargo

echo '[target.i686-unknown-linux-musl]' >>.cargo/config
echo 'linker = "i686-linux-musl-gcc"' >>.cargo/config

echo '[target.aarch64-unknown-linux-musl]' >>.cargo/config
echo 'linker = "aarch64-linux-musl-gcc"' >>.cargo/config

cargo build -Zmultitarget --target=x86_64-unknown-linux-musl --target=i686-unknown-linux-musl --target=aarch64-unknown-linux-musl --release -vv

cd target/x86_64-unknown-linux-musl/release/ && tar -cvf react-x86_64-unknown-linux-musl.tar.gz react && cd ../../..
cd target/i686-unknown-linux-musl/release/ && tar -cvf react-i686-unknown-linux-musl.tar.gz react && cd ../../..
cd target/aarch64-unknown-linux-musl/release && tar -cvf react-aarch64-unknown-linux-musl.tar.gz react && cd ../../..
