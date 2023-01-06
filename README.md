# Lilo's Operating system

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

