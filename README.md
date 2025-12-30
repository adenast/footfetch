# Footfetch

CLI utility for viewing system information, but with a twist.

---

## Screenshots

<p align="center"> <img src="https://raw.githubusercontent.com/lobotomydev/footfetch/main/images/screenshot.png" alt="Screenshot" width="600"> </p>

<div align="center">
    <small><i>Footfetch 1.4.0 on Arch Linux with GNOME</i></small>
</div>

---

## Installation, Usage and How to build

## Installation

> **⚠️ Important:** The project requires **curl** to install.

**To install** just run this command in your terminal:
```bash
curl -o install.sh https://raw.githubusercontent.com/adenast/footfetch/main/install.sh
chmod +x install.sh
./install.sh
rm install.sh
```

## How to use

```bash
footfetch
```

## Uninstall

```bash
curl -o uninstall.sh https://raw.githubusercontent.com/adenast/footfetch/main/uninstall.sh
chmod +x uninstall.sh
./uninstall.sh
rm uninstall.sh
```


---

## Introduction

Compiling your own version of the program is essential because it was originally designed for Linux, but its architecture allows it to run on all UNIX-like operating systems.

Here is why this is important:

Portability. The source code is not tied to a specific Linux version or distribution. By compiling it yourself, you adapt the program to your system, whether it's macOS, FreeBSD, or another UNIX-like OS.

Up-to-dateness. You can always compile the latest version of the program, including recent bug fixes and new features that haven't yet been included in official builds.

## Prerequisites

**You need** GCC (or other C compiler, for example: clang) and program source code.

### How to get program source code:
Clone this repository (requires `git`):
```bash
git clone https://github.com/adenast/footfetch/
``` 
or

Download program source code (requires `curl`, takes up less disk space)
```bash
curl -o footfetch.c https://raw.githubusercontent.com/adenast/footfetch/main/footfetch.c
```

## The Build Process

**Using GCC:**
```bash
gcc --Wall -g -o footfetch footfetch.c
```
**Using Clang:**
```bash
clang --Wall -g -o footfetch footfetch.c
```

**Now** you can use program by command (Works only in the repository where exist program executable file):
```bash
./footfetch
```

## Advanced Topics
If you want the program to be available globally (from any directory), you need to add the program's executable file to the PATH system variable.

The easiest way is to copy or move your executable to one of the directories that is already in PATH.
Here are some standard locations that are usually included in this variable:

`/usr/local/bin`
`/usr/bin`
`/bin`
`~/.local/bin` (user only)

---

## LICENSE

This project is licensed under the MIT license. For details, see the file [LICENSE.md](https://github.com/lobotomydev/footfetch/blob/main/LICENSE.md).

---
