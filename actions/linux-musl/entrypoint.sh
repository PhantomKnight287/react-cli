#!/usr/bin/env bash

cargo build --release -vv
cd target/release/ && tar -cvf react-x86_64-unknown-linux-musl.tar.gz react && cd ../..
