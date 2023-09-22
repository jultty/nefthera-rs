# nefthera

Nefthera is a text-based RPG inspired by classical MUDs.

## Installing

This game is still in early development. For now, you need to have Rust set up on your machine to build and run it.  The easiest way to do that is to use [rustup](https://rustup.rs/). 

Once you have Rust set up, you can clone or download this repository and build:

```sh
git clone https://github.com/jultty/nefthera
cd nefthera
cargo build --release
```

This will install all dependencies and build the game. The binary should be at `target/release/nefthera`.

## Running

Run the binary with the `--start` argument, which initiates the main game loop:

```sh
./nefthera --start
```
