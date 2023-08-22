# Wondercard Beacon

[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-darkgreen.svg)](https://opensource.org/license/gpl-3-0/)

Wondercard Beacon is a command-line application to distribute Pokémon generation IV wondercards over Wi-Fi using a
computer.

## Features

- Distributing wondercards via Wi-Fi
- Decrypting wondercards dumped from distributions

## Installation

To install Wondercard Beacon, you will need to have Rust and Cargo installed. You can install them
using [rustup](https://rustup.rs/).

Once you have Rust and Cargo installed, follow these steps:

1. Clone this repository: `git clone https://github.com/Eiskasten/wc-beacon.git`
2. Navigate to the project directory: `cd wc-beacon`
3. Build the application: `cargo build --release`

## Usage

Currently, Wondercard Beacon is only tested on Linux.
However, it is very likely, that it will work on other operating systems as well.
If you manage to get it to run on an operating system different from Linux, a pull request with a tutorial is
appreciated.

Before you can use the application, you need to make a few preparations.

1. Put your Wi-Fi card into monitor mode
2. Listen to Wi-Fi channel 7

> :warning: Your Wi-Fi card will not be able to retain the internet connection, when in monitor mode.
> If you need an internet connection during the procedure, make sure your computer has an additional network interface.

Please research for yourself how to achieve these requirements.
However, for Linux users, a [script](scripts/prepare-wifi.sh) is available for that.
Just run it using: `sudo ./scripts/prepare-wifi.sh <device>`.

The restriction for channel 7 may be removed in the future.

After these steps, you may finally distribute your wondercards using the example below.

## Examples

Here are some example usages of Wondercard Beacon:

```sh
# Distribute the membercard using wlp0s20f3, requires root/sudo
cargo run -- dist -p membercard.pcd -r en -d wlp0s20f3
# You can then receive the mystery gift in your pokemon game.

# Decrypt encrypted membercard
cargo run -- dec -e membercard.pcd.enc -c 1cb4 -a a4:c0:e1:6e:76:80 -p decryped.pcd
```

It might be possible that commands which require root/sudo must be run with `sudo ./target/release/wc-beacon <arguments>` instead of `sudo cargo run -- <arguments>`.

For further options, run `cargo run -- dist --help` and `cargo run -- dec --help`.

## Links

- https://github.com/projectpokemon/EventsGallery: Repository with many wondercards
- https://gbatemp.net/threads/reverse-engineering-pokemon-gen4-wonder-card-wi-fi-distribution.488212: Thread which
  explains the distribution structure

## Other Pokémon Generations

Currently, only generation IV is supported by this application.
However, since the generation V data structure of distributions seem quite similar - although not the same - it may be
possible to distribute generation V wondercards in the future.

## Contributing

Contributions are welcome!
If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.
Pull requests are also appreciated.

## Credits

- [Yuuto](https://gbatemp.net/members/yuuto.435475/): reverse-engineered the distribution process
- [Eiskasten](https://github.com/Eiskasten): implemented this application

## License

This project is licensed under the GPLv3 License - see the [LICENSE.md](LICENSE.md) file for details.
