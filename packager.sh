#!/bin/bash

read -p "Current version: " version
rm -rf ~/.tmp/sherlock-release/
mkdir -p ~/.tmp/sherlock-release/
cargo build --release
cp target/release/sherlock ~/.tmp/sherlock-release/sherlock
cp LICENSE ~/.tmp/sherlock-release/LICENSE

cd ~/.tmp/sherlock-release/
tar -czf sherlock-v${version}-bin-linux-x86_64.tar.gz sherlock LICENSE

