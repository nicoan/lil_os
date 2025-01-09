# Lilo's Operating system

This is a hobby OS that I am doing just for fun and learning. The idea is to implement the a basic kernel for x86_64 architecture and on top of that add a compatibility layer for DOS programs and implement the Win 3.x API to run Win 3.x programs "natively" (for the sake of nostalgia :P).

I usually work on this project by periods (spend some time on it and leave it for several months), because I either have another side project that I am interested at the moment or I am too busy to work on side projects.

Almost everything is based off the great [Phillipp Oppermann blog](https://os.phil-opp.com/), but I try to reimplement the "black bloxes" (such as the x86_64 crate). My goal is to not have external dependencies (and with the benefit that this way, I learn the how things work on a deeper level).

# Setting up environment

First you need to install some tools:

### Install bootimage
```bash
$ cargo install bootimage
```

### Instal needed components
```bash
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
$ rustup component add llvm-tools-preview
```

### Use nightly version

```bash
$ rustup override set nightly
```

**NOTE**: A lot of code-comments are directly taken from https://os.phil-opp.com/

## Build
```
$ cargo bootimage
```

## Run
You need to have QEMU installed

```
$ cargo run
```

