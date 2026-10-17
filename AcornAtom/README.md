On the highest level is a workspace, which is a manually created folder with a Cargo.toml:
[workspace]
members = [
    ....
]
The workspace folder is the project and should have a name reflecting that e.g. Amazing_game

The members are packages created in the workspace with:
Shell:
$ cargo new <package_name>

Cargo will create the skeleton packages, e.g.
Amazing_game
    engine
        Crate.toml
        src
            engine.rs
        test

With the package engine defined by its Cargo.toml e.g.
[package]
name = "engine"
version = "0.1.0"
edition = "2024"

[dependencies]

Two types of crates, that make up a package are: Binary and Library, indicating what they result in.
Binary crates result in an executable with the package name:
Amazing_game
    engine
        target #possibly a cross target: target/arm-unknown-linux-gnueabihf
            debug
                engine # <- executable

An ordinary function definition in a .rs file will give this.

Library crates produce .rlib libraries