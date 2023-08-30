# gobble
Rust rewrite of Devour

Gobble hides your current window before launching an external program and unhides it after quitting. Useful in terminals and file managers to keep your screen uncluttered.

## Installation

### Arch

```
git clone https://github.com/EmperorPenguin18/gobble
cd gobble
makepkg -si
```
Or just install from the [AUR](https://aur.archlinux.org/packages/gobble)

### Nix and NixOS

Install from [Nixpkgs Unstable](https://search.nixos.org/packages?channel=unstable&from=0&size=50&sort=relevance&type=packages&query=gobble).

### Other Linux

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
Read the man page for more info

## Pro Tip

To automate using gobble:

Find and edit your shell rc (Usually ~/.bashrc)

Add this to it:

`alias mpv='gobble mpv'`

`alias vimiv='gobble vimiv'`

for example to automatically swallow when you open something in mpv or vimiv from CLI.

In pcmanfm (and probably other gui file managers):  you can right click on a file -> open with -> custom command line. Under command line to execute:

`gobble mpv %f`

Enter 'mpv' in 'Application name (optional, set it to keep association)'.

Then check 'Set selected application as default for this file type', and click ok.

## Uninstallation

Arch

```
pacman -R gobble #as root
```

Other Linux

```
cd gobble
make uninstall #as root
```
