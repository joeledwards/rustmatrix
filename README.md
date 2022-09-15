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
    -c, --colors <COLORS>
            Weighted sequence of colors (repeat color character for greater weight). (valid chars
            are: b, c, g, k, m, r, w, y [env: TERMATRIX_COLORS=] [default: g]

    -d, --min-step-delay <MIN_STEP_DELAY>
            Minimum value of trace (column) scroll delay in milliseconds. [env:
            TERMATRIX_MIN_STEP_DELAY=] [default: 40]

    -D, --max-step-delay <MAX_STEP_DELAY>
            Maximum value of trace (column) scroll delay in milliseconds. [env:
            TERMATRIX_MAX_STEP_DELAY=] [default: 120]

    -f, --update-frequency <UPDATE_FREQUENCY>
            Refresh rate (in Hz) for terminal updates. [env: TERMATRIX_UPDATE_FREQUENCY=] [default:
            60]

    -h, --help
            Print help information
```
