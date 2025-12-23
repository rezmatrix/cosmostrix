# Cosmostrix

Cosmostrix is a terminal "Matrix rain" visualizer written in Rust.

It is a clean-room Rust migration of an older ncurses-based terminal project.

## Build

```bash
cargo build --release
./target/release/cosmostrix --help
```

## Installation

Cosmostrix is intended for Unix-like systems (Linux, BSD, macOS, and similar).

### From source

```bash
cargo build --release
install -Dm755 ./target/release/cosmostrix ~/.local/bin/cosmostrix
```

### From GitHub Releases

Download the `.tar.xz` archive for your OS/arch from Releases, extract it, and place `cosmostrix` somewhere in your `PATH`.

## Run

```bash
cargo run -- --color green --fps 60 --speed 10
```

Cosmostrix runs in **alternate screen** and **raw mode**.

## CLI options

These flags match the current Rust implementation (`src/config.rs`).

```text
 -a, --async                  enable async column speeds
 -b, --bold <NUM>             0=off, 1=random, 2=all
 -C, --colorfile <FILE>       load user colors from file (legacy-compatible format)
 -c, --color <COLOR>          color scheme (default: green)
 -D, --defaultbg              use terminal default background color
 -d, --density <NUM>          droplet density (default: 1.0)
 -F, --fullwidth              use two columns per character
 -f, --fps <NUM>              target FPS (default: 60)
 -g, --glitchms <LO,HI>       glitch timing range in ms (default: 300,400)
 -G, --glitchpct <PCT>        glitch chance percent (default: 10)
 -l, --lingerms <LO,HI>       linger timing range in ms (default: 1,3000)
 -M, --shadingmode <NUM>      0=random, 1=distance-from-head (default: 0)
 -m, --message <TEXT>         overlay message
     --maxdpc <NUM>           max droplets per column (default: 3)
     --noglitch               disable glitch
 -r, --rippct <PCT>           die-early percent (default: 33.33333)
 -S, --speed <NUM>            chars per second (default: 8)
 -s, --screensaver            exit on first keypress
     --shortpct <PCT>         short droplet percent (default: 50)
     --charset <NAME>         character set (default: auto)
     --chars <HEX...>         custom unicode hex ranges (pairs)
     --colormode <MODE>       force color mode (0, 16, 256, 32)
```

### Color schemes

`--color` supports:

`user`, `green`, `green2`, `green3`, `gold`, `yellow`, `orange`, `red`, `blue`, `cyan`, `purple`, `pink`, `pink2`, `vaporwave`, `gray`, `rainbow`

If `--colorfile` is provided, Cosmostrix automatically switches to `user` color scheme.

### Background

By default Cosmostrix draws with an explicit black background.

If you want it to use your terminal theme background, pass `--defaultbg`.

## Runtime controls (keys)

Controls are handled in `src/main.rs`:

```text
 Esc / q      quit
 Space        reset
 a            toggle async mode
 p            pause/unpause
 Up/Down      change speed
 Left/Right   change glitch percent
 Tab          toggle shading mode
 - / +        change density

 1..0 ! @ # $ %   switch color schemes
```

## Reference folder

`reference/` contains an older version of the project kept for comparison during migration.

## Notes

- **Terminal compatibility**: best results in modern terminals with 256-color or truecolor support.
- **UTF-8**: Cosmostrix can use Unicode character sets depending on your locale and `--charset`.
