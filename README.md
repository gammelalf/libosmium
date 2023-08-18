# Libosmium

[![GitHub](https://img.shields.io/badge/github-libosmium-8da0cb?logo=github)](https://github.com/gammelalf/libosmium)
[![crates.io version](https://img.shields.io/crates/v/libosmium.svg)](https://crates.io/crates/libosmium)
[![docs.rs docs](https://docs.rs/libosmium/badge.svg)](https://docs.rs/libosmium)
[![crates.io version](https://img.shields.io/crates/l/libosmium.svg)](https://github.com/gammelalf/libosmium/blob/master/LICENSE)
[![CI build](https://github.com/gammelalf/libosmium/workflows/CI/badge.svg)](https://github.com/gammelalf/libosmium/actions)

Rust binding and wrapper around the excellent [libosmium](https://osmcode.org/libosmium/) c++ library.

## Maintenance

This crate was started out of necessity for a larger project.
Therefore, it won't get much attention unless I need another feature or find a bug.
But feel free to contribute.

## What it does

This crate exposes libosmium's osm object classes (i.e. `OSMObject`, `Node`, `Way`, etc.)
and the `Handler` interface to read those from a file (currently only `.pbf`).

Since libosmium has its own memory management, all objects are only exposed via references.
So most of the types on rust's side are empty enums which can't be instantiated.

To expose these c++ classes' methods, this crate uses a small c++ shim (namely `src/libosmium.cpp`)
which reexports them as un-mangled functions taking pointers.
Methods whose behaviour is trivial enough are simply implemented directly in rust to avoid unnecessary boilerplate.

## Development
* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`. Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.
* On `git push`, it will run a few validations, including `cargo fmt`, `cargo clippy`, and `cargo test`.  Use `git push --no-verify` to skip these checks.

#### Build dependencies

* This repo uses submodules.  To clone it, use `git submodule update --init --recursive`.
* This package builds libosmium and therefore needs its [dependencies](https://osmcode.org/libosmium/manual.html#dependencies).

Install for debian:
```bash
apt install build-essential libboost-dev libprotozero-dev zlib1g-dev
```

Install for arch:
```bash
pacman -Sy cmake make boost-libs protozero zlib
```
