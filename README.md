# jabba-cpc

The name "cpc" stands for _**c**opy **p**ath to **c**lipboard_. This is a binary crate.

`cpc` copies the path of the current working directory to the clipboard.
If a parameter is given, it's also added to the path.

Supported platforms: Windows and Linux (with X server).

## Demo

<div align="center">
  <a href="https://www.youtube.com/watch?v=iNdqZzs79FU"><img width="60%" src="https://raw.githubusercontent.com/jabbalaci/jabba-cpc/main/assets/youtube.jpg" alt="view demo on YouTube"></a>
</div>

## Windows

Put `cpc.exe` to a folder that is in your PATH.

Example:

```
c:\> cd c:\download

c:\download> cpc
```

Now the current directory's path, "**c:\download**" is copied to the clipboard.

```
c:\download> cpc tree.jpg
```

Now the absolute path of the given file, "**c:\download\tree.jpg**" is copied to the clipboard.

Help: `cpc.exe -h`

## Linux

`cpc` relies on the external command `xsel` to manipulate the content of the clipboard.
Thus, you must install `xsel` using your package manager (under Ubuntu it's
`sudo apt install xsel`).

Under Linux, there are two clipboards. They are called "primary" and "clipboard". `cpc`
puts the path on both of them, thus you can insert the path with one of the following
methods: Ctrl+v, Shift+Insert, or mouse middle click.

See the example above, it works similarly under Linux.

Help: `cpc -h`

## Installation

`cpc` is written in Rust. If you have the Rust compiler, you can install it directly
from crates.io using the command `cargo`:

    $ cargo install jabba-cpc

## Links

* [jabba-ctc](https://github.com/jabbalaci/jabba-ctc): **c**opy **t**ext to **c**lipboard
* [go-cpc-ctc](https://github.com/jabbalaci/go-cpc-ctc): a Go implementation of cpc and ctc
