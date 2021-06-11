# gobble
Rust rewrite of Devour

Gobble hides your current window before launching an external program and unhides it after quitting. Useful in terminals and file managers to keep your screen uncluttered.

Currently only works with X but I would like to add Wayland support in the future.

## Installation

Arch

```
git clone https://github.com/EmperorPenguin18/gobble
cd gobble
makepkg
```

Other Linux

```
git clone https://github.com/EmperorPenguin18/gobble
cd gobble
make
make install #as root
```

## Usage

```
gobble CMD ...
```
