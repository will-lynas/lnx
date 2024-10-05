# lnx 🔥

A *very* thin wrapper around `ln -s` 🔗

## Features

- More intuitive argument names
- Resolves all paths
- Creates parent directories if needed

## Install

```sh
cargo install --git https://github.com/will-lynas/lnx
```

## Usage

```sh
$ lnx --help
Creates symbolic links with resolved absolute paths.

Usage: lnx --real-path <REAL_PATH> --fake-path <FAKE_PATH>

Options:
  -r, --real-path <REAL_PATH>  The target path the symlink will point to (real path)
  -f, --fake-path <FAKE_PATH>  The path where the symlink will be created (fake path)
  -h, --help                   Print help
```
