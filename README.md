# Footfetch

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![AUR version](https://img.shields.io/aur/version/footfetch-git?color=blue)](https://aur.archlinux.org/packages/footfetch-git)
[![Nix Built](https://img.shields.io/badge/Nix-Flake-blueviolet)](https://nixos.org)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)


**CLI** utility for viewing **system information**, but with a **twist**.

<p align="center"> <img src="https://raw.githubusercontent.com/adenast/footfetch/main/images/screenshot.png" alt="Screenshot" width="600"> </p>

<div align="center">
    <small><i>Footfetch 1.40</i></small>
</div>

---

## Installation

##### Arch:
```bash
yay -S footfetch-git
```
##### NixOS:
```bash
nix run github:adenast/footfetch
```
##### Other:
```bash
curl -o install.sh https://raw.githubusercontent.com/adenast/footfetch/main/scripts/install.sh
chmod +x install.sh
./install.sh
rm install.sh
```
##### From Source:
```bash
git clone https://github.com/adenast/footfetch && cd footfetch
cargo install --path .
```

---

## LICENSE

This project is licensed under the MIT license. For details, see the file [LICENSE.md](https://github.com/lobotomydev/footfetch/blob/main/LICENSE.md).

---
