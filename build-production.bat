@echo off

call cargo build --release

copy /y target/release/pmv-cli.exe pmv-cli.exe
