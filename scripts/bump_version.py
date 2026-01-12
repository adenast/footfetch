#!/usr/bin/env python3

import json
import re
import subprocess
from pathlib import Path
import sys

VERSION_FILE = Path(__file__).resolve().parents[1] / "version.json"

FILES_TO_PATCH = [
    "Cargo.toml",
    "PKGBUILD",
    "flake.nix",
]


def get_new_version() -> str:
    if not VERSION_FILE.is_file():
        print(f"Version file not found: {VERSION_FILE}", file=sys.stderr)
        sys.exit(1)

    data = json.loads(VERSION_FILE.read_text(encoding="utf-8"))
    version = data.get("version", "").strip()

    if not re.match(r"^\d+\.\d+\.\d+$", version):
        print(f"The version has an unexpected format: {version}", file=sys.stderr)

    return version

def patch_file(rel_path: str, new_version: str) -> bool:
    path = Path.cwd().parent / rel_path

    if not path.is_file():
        return False

    original = path.read_text(encoding="utf-8")
    content = original

    if path.name == "Cargo.toml":
        content = re.sub(
            r'^version\s*=\s*["\'][\d.]+["\']',
            f'version = "{new_version}"',
            content, flags=re.MULTILINE
        )

    elif path.name == "PKGBUILD":
        content = re.sub(
            r'^pkgver=.*$',
            f"pkgver={new_version}",
            content, flags=re.MULTILINE
        )

    elif path.name == "flake.nix":
        content = re.sub(
            r'version\s*=\s*["\'][\d.]+["\']',
            f'version = "{new_version}"',
            content, flags=re.MULTILINE
        )
        content = re.sub(
            r'\b(v?[\d.]+)\b(?=.*(rev|tag|ref)\s*=)',
            new_version,
            content, flags=re.MULTILINE
        )

    if content != original:
        path.write_text(content, encoding="utf-8")
        return True

    return False

def update_cargo_lock() -> bool:
    try:
        subprocess.run(["cargo", "update", "--quiet"], check=True)
        return True
    except Exception:
        return False

def regenerate_srcinfo() -> bool:
    root = Path(__file__).resolve().parent.parent
    
    try:
        result = subprocess.run(
            ["makepkg", "--printsrcinfo"],
            cwd=root,
            check=True,
            capture_output=True,
            text=True
        )
        
        (root / ".SRCINFO").write_text(result.stdout, encoding="utf-8")
        return True
        
    except Exception as e:
        return False

def main():
    new_version = get_new_version()

    updated = []
    for rel_path in FILES_TO_PATCH:
        if patch_file(rel_path, new_version):
            updated.append(rel_path)

    cargo_updated = update_cargo_lock()
    srcinfo_updated = regenerate_srcinfo()

if __name__ == "__main__":
    main()