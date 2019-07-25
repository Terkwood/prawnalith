#!/bin/bash

cargo clean
rm -rf /usr/local/cargo/registry
rm -rf /usr/local/cargo/git
rustup toolchain list|xargs rustup toolchain uninstall
