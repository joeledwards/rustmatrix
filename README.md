# termatrix
Cli application to simulate the display from "The Matrix" in terminal. Based on [rustmatrix](https://github.com/meganehouser/rustmatrix).

<img width="726" alt="Screen Shot 2022-05-01 at 11 54 39" src="https://user-images.githubusercontent.com/412853/166158329-bd5fe01a-bcf1-4a98-932c-cc6149675786.png">

## Installation

### Cargo
```shell
cargo install termatrix
```

### NetBSD
```shell
pkgin install termatrix
```

## Usage

```shell
$ termatrix --help
termatrix

USAGE:
    termatrix [OPTIONS]

OPTIONS:
    -c, --glyph-color <COLOR>    [env: TERMATRIX_GLYPH_COLOR=] [default: green]
    -d, --delay-ms <DELAY_MS>    [env: TERMATRIX_DELAY_MS=] [default: 50]
    --density <DENSITY>          [env: TERMATRIX_DELAY_MS=] [default: 0.5]
    -h, --help                   Print help information
```
