<br/>
<p align="center">
  <h3 align="center">Ferris Say</h3>

  <p align="center">
    A simple CLI made with rust
    <br/>
    <br/>
  </p>
</p>

![Contributors](https://img.shields.io/github/contributors/CinematicCow/ferris-say?color=dark-green) ![Issues](https://img.shields.io/github/issues/CinematicCow/ferris-say) ![License](https://img.shields.io/apm/l/vim-mode)

## About The Project

![Screen Shot](https://i.imgur.com/LTMP49g.png)

Ferris-Say is a simple cli tool, made with **Rust**. It works like the popular tool cowsay. This project was made to understand the basic concepts of rust like `structs, enums, match` and `CLI input handeling` with `clap`.

## Built With

This project was bootstrapped with `cargo new --bin`.
The dependency crates are `clap` and `ansi_term`.

## Getting Started

To get a local copy up and running follow these simple example steps.

_Note : The following presumes you are on a Linux system or Windows Subsystem for Linux_

### Prerequisites

The following things are required to build it locally

- rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. Clone the project

```sh
git clone https://github.com/CinematicCow/ferris-say.git
```

2. Fetch the required crates and build the project

```sh
cargo build
```

3. Change directory to the generated binary

```sh
cd target/debug/
```

4. Run the project

```sh
./ferris-say --help
```

## Authors

- **CinematicCow** - _Software Engineer_ - [CinematicCow](https://github.com/CinematicCow) - \*\*

## Acknowledgements

Thank you spaghettidev for the awesome tutorial.

- [spaghettidev ](https://spaghettidev.tech/)
