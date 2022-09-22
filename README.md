# termatrix
Cli application to simulate the falling glyphs from "The Matrix" in your terminal (forked from [rustmatrix](https://github.com/meganehouser/rustmatrix)).

<img width="600" alt="tematrix color scrolling GIF" src="https://user-images.githubusercontent.com/412853/191857343-43f8ea6f-7c70-452c-a958-55c3b3654683.gif">

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

## Customization

Defaults to green glyphs:
```
termatrix
```
<img width="726" alt="termatrix green" src="https://user-images.githubusercontent.com/412853/166158329-bd5fe01a-bcf1-4a98-932c-cc6149675786.png">


You can select a combination of colors via the --colors/-c option:
```
termatrix -c rgbcmy
```
Or via the `TERMATRIX_COLORS` environment variable:
```
TERMATRIX_COLORS=rgbcmy termtarix
```
<img width="795" alt="termtarix color" src="https://user-images.githubusercontent.com/412853/191857267-2de948cc-4a59-49f9-aeae-2276c56a302e.png">
