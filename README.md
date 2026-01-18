<div align="center">

# footfetch

[![Commits](https://img.shields.io/github/commit-activity/m/adenast/footfetch?style=for-the-badge&labelColor=1e2528&color=blue)](https://github.com/adenast/footfetch/commits)
[![Build](https://img.shields.io/github/actions/workflow/status/adenast/footfetch/release.yml?style=for-the-badge&labelColor=1e2528)](https://github.com/adenast/footfetch/actions)
[![License](https://img.shields.io/github/license/adenast/footfetch?style=for-the-badge&labelColor=1e2528&color=violet)](https://opensource.org/licenses/MIT)
[![Stars](https://img.shields.io/github/stars/adenast/footfetch?style=for-the-badge&labelColor=1e2528&color=yellow)](https://github.com/adenast/footfetch/stargazers)
[![Issues](https://img.shields.io/github/issues/adenast/footfetch?style=for-the-badge&labelColor=1e2528&color=gray)](https://github.com/adenast/footfetch/issues)

<img src="https://github.com/adenast/footfetch/blob/main/images/preview.png?raw=true" alt="Preview" width="600"> 

<small><i>Footfetch 1.42.4 on Termux</i></small>

</div>

---

## About
* A **lightweight and high-performance**, **neofetch-like** tool for those who prefer **feet over faces** written in **Rust**. 

## Features
 * **Zero Dependencies:** Single static binary; no external libraries or runtimes required.
 * **Live Mode:** Real-time hardware monitoring via the `--live` flag.
 * **Deep Fetch:** Accurate CPU/GPU usage and model detection.
 * **Multi-Distro:** Native package counting for Pacman, Dpkg, RPM, and APK.

## Installation
### Quick Start (Binary)
For those who want it right now (pre-compiled for x86_64):
```Bash
curl -sSL https://raw.githubusercontent.com/adenast/footfetch/main/scripts/linux/x86_64/install.sh | sh
```

### Source-based
Recommended for performance. These methods will build the app specifically for your system

 * Arch Linux (AUR):
```Bash
yay -S footfetch-git
```
 * NixOS (Flakes):
```Bash
nix run github:adenast/footfetch
```
 * Cargo:
```Bash
cargo install footfetch
```

## License

This project is licensed under the MIT license. For details, see the file [LICENSE.md](https://github.com/adenast/footfetch/blob/main/LICENSE.md).