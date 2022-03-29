#!/usr/bin/env bash
mkdir .cargo

echo '[target.i686-unknown-linux-musl]' >>.cargo/config
echo 'linker = "i686-linux-musl-gcc"' >>.cargo/config

echo '[target.aarch64-unknown-linux-musl]' >>.cargo/config
echo 'linker = "aarch64-linux-musl-gcc"' >>.cargo/config

echo '[target.armv7-unknown-linux-musleabi]' >>.cargo/config
echo 'linker = "arm-linux-musleabi-gcc"' >>.cargo/config

echo '[target.armv7-unknown-linux-musleabihf]' >>.cargo/config
echo 'linker = "arm-linux-musleabihf-gcc"' >>.cargo/config

cargo build -Zmultitarget --target=x86_64-unknown-linux-musl \
    --target=i686-unknown-linux-musl \
    --target=aarch64-unknown-linux-musl \
    --target=armv7-unknown-linux-musleabi \
    --target=armv7-unknown-linux-musleabihf \
    --release -vv

cd target/x86_64-unknown-linux-musl/release/ && tar -cvf react-x86_64-unknown-linux-musl.tar.gz react && cd ../../..
cd target/i686-unknown-linux-musl/release/ && tar -cvf react-i686-unknown-linux-musl.tar.gz react && cd ../../..
cd target/aarch64-unknown-linux-musl/release && tar -cvf react-aarch64-unknown-linux-musl.tar.gz react && cd ../../..
cd target/armv7-unknown-linux-musleabi/release/ && tar -cvf react-armv7-unknown-linux-musleabi.tar.gz react && cd ../../..
cd target/armv7-unknown-linux-musleabihf/release/ && tar -cvf react-armv7-unknown-linux-musleabihf.tar.gz react && cd ../../..
