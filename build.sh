#!/bin/bash

cargo build --release
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-apple-darwin

zip rcpd_linux.zip ./target/release/rcpd
zip rcpd_windows.zip ./target/x86_64-pc-windows-gnu/release/rcpd.exe
zip rcpd_osx.zip ./target/x86_64-apple-darwin/release/rcpd
