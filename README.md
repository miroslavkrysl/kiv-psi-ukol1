# KIV/PSI - úkol 1 

Author: Miroslav Krýsl

## Description

Simple SSDP client. It just sends multicast SSDP discovery request a receives responses.
There is a timeout to wait for all responses - 8 seconds.

## Run

Binaries for Linux and Windows are included in `bin` directory.

It works well on linux, but there is a problem on Windows, because it sometimes works and sometimes doesn't.
It seems it is because Windows assigns address from a random interface (including the virtual ones) for the UDP multicasts
[source: multiple google/stackoverflow searches].

## Install, build, run

It is written in Rust. Rust projects are managed with `cargo` command which needs
the `Cargo.toml` file in project root.

You can install Rust toolchain with your preferred way, but the simplest way
is to install Rust via `rustup` (https://rustup.rs/). You should follow instructions
on the website and then instructions in the installation script.

If you have Rust installed, just run in project root:

`cargo run` for building and running.


Or only for building:

`cargo build`
