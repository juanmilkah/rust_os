# Rust Minimal Operating System

To compile the program for bare metal target run:

```bash
cargo build --target thumbv7em-none-eabihf
```

To Compile for host system by passing additional linker arguments run:

## Linux

```bash
cargo rustc -- -C link-arg=-nostartfiles
```

## Windows

```bash
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
```

## MacOS

```bash
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
