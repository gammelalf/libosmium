# Libosmium

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

## Build dependencies

This package builds libosmium and therefore needs its [dependencies](https://osmcode.org/libosmium/manual.html#dependencies).

Install for debian:
```bash
apt install build-essential libboost-dev libprotozero-dev zlib1g-dev
```

Install for arch:
```bash
pacman -Sy cmake make boost-libs protobuf zlib
```
