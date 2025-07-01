# CHIP-8 Emulator

A CHIP-8 emulator written in Rust.

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run path/to/rom.ch8
```

## Test ROM

A Tetris ROM is included in the `testrom/` directory for testing.

```bash
cargo run testrom/tetris.rom
```

## Requirements

- Rust 1.56 or later