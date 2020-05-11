# rusty-brightness
Simple brightness manager helper tool made as as a rust practice project.

Main use: set brightness of multiple display with single command.

Note: if you just want a helper for xrandr with multiple monitors, check out https://github.com/Ventto/mons

## Requirements
 - [xrandr](https://gitlab.freedesktop.org/xorg/app/xrandr)
 - [rust](https://www.rust-lang.org/learn/get-started)

## Installation
In repository root:

development: ```cargo build``` -> target/debug/rusty-brightness

optimized: ```cargo build --release``` -> target/release/rusty-brightness

##### Enable anywhere in terminal
After build: 

move the file to directory specified in $PATH

```cp target/release|debug/rusty-brightness <path-sepecifiend in $PATH>```

e.g. ```sudo cp target/release|debug/rusty-brightness /usr/local/bin```

or without sudo:


create a local user directory for binaries 
 
```mdkir -p $HOME/bin```

add it to path in shell rc file unless you already have made one.
 - the example is for bash (bourne again shell)
 - show shell name in terminal when in doubt: ```echo $0```
 
```echo 'export PATH="$PATH:$HOME/bin"' > $HOME/.bashrc```

source the rc file to apply changes in current terminal 

``` . $HOME/.bashrc```

move the file 

```mdkir -p $HOME/bin && cp target/release|debug/rusty-brightness $HOME/bin```

## Usage

##### examples
change all monitors' brightness to half:

```rusty-brightness -a -b 0.5```

change first two monitors' brightnesses:

```rusty-brightness -i 1 2 -b .8 .7```

##### displayed with ```rusty-brightness -h```

```
USAGE:
    rusty-brightness [FLAGS] [OPTIONS] --brightness <BRIGHTNESS>...

FLAGS:
    -a, --all        
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --brightness <BRIGHTNESS>...          floating point value between 0 and 1 [default: 0.5]
    -i, --display_index <display_index>...    integer
    -n, --display_name <display_name>...      string point value
    -v, --verbosity <verbosity>               Sets the level of verbosity [default: 0]
```
