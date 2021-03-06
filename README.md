# blockz

![Issues](https://img.shields.io/github/issues/mcb2003/blockz)
![Forks](https://img.shields.io/github/forks/mcb2003/blockz)
![Stars](https://img.shields.io/github/stars/mcb2003/blockz)
![License (MIT)](https://img.shields.io/github/license/mcb2003/blockz)

Blockz is an accessible puzzle game with the goal of covering targets with movible blocks.
It is basically an extended version of [Javidx9's sliding block puzzle game](https://www.youtube.com/watch?v=l7YEaa2otVE)
and even uses [Rust bindings](https://crates.io/crates/olc_pixel_game_engine)
to his [One Lone Coder Pixel Game Engine](https://github.com/OneLoneCoder/olcPixelGameEngine).
Well ... it *will* be that, once it's finished, anyway.

## A Note on OS Compatibility

Currently, Blockz only runs on Linux. This is only temporary: I need to figure out what's going wrong on macOS and fix it (likely something to do with the
[tts crate](https://crates.io/crates/tts)). Windows support will come if and when the
[olc_pixel_game_engine crate](https://crates.io/crates/olc_pixel_game_engine) gains Windows support.

## Compile and Run

```bash
git clone https://github.com/mcb2003/blockz.git
cd blockz
# Compile (binary is in target/release/blockz)
cargo build --release
# Compile and run
cargo run --release
```
