@echo off

call cargo build --release

call cp -f target/release/pmv-cli.exe pmv-cli.exe
