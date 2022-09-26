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
            Weighted sequence of colors (repeat color character for greater weight). Valid chars
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

    -g, --glyph-set <GLYPH_SET>
            The set of glyphs which should be displayed. [env: TERMATRIX_GLYPH_SET=] [default: all]
            [possible values: all, alpha, alpha_lower, alpha_upper, alphanum, binary, decimal, hex,
            hex_lower, hex_upper, special]

    -h, --help
            Print help information
```

## Customization

## Default

Defaults to green glyphs:
```
termatrix
```
<img width="726" alt="termatrix green" src="https://user-images.githubusercontent.com/412853/166158329-bd5fe01a-bcf1-4a98-932c-cc6149675786.png">

## Colors

You can select a combination of colors via the --colors/-c option:
```
termatrix -c rgbcmy
```
Or via the `TERMATRIX_COLORS` environment variable:
```
TERMATRIX_COLORS=rgbcmy termtarix
```
<img width="795" alt="termtarix color" src="https://user-images.githubusercontent.com/412853/191857267-2de948cc-4a59-49f9-aeae-2276c56a302e.png">

## Glyph Sets

You can select the subset of glyphs to display via the --glyph-set/-g option:
```
termatrix -g binary
```

Or via the `TERMATRIX_GLYPH_SET` environment variable:
```
TERMATRIX_GLYPH_SET=special termtarix
```

This adjusts which characters compose the alphabet from which each trace can select.

For example, the `binary` glyph set only includes the characters `0` and `1`.

## Step Delays

Step delay is a minimum duration between updates to a trace (vertial cluster of glyphs). The step delay for each trace is a randomly selected duration between the `-min-step-delay` and the `--max-step-delay`.

The variation in step delays results in faster traces passing slower traces. If `--min-step-delay` and `--max-step-delay` are the same, all traces will move at the same rate.
