#!/usr/bin/env python3
"""
NVDA Fighter Build Script
Compiles Rust crates for x86 (i686) and x64 (x86_64) MSVC/GNU and packages the .nvda-addon bundle.
"""

import os
import sys
import shutil
import zipfile
import subprocess
import argparse

TARGET_MAP = {
    "x86": "i686-pc-windows-msvc",
    "x64": "x86_64-pc-windows-msvc",
    "arm64": "aarch64-pc-windows-msvc",
}

def build_rust_target(arch):
    target_triple = TARGET_MAP.get(arch)
    print(f"[*] Building Rust crates for {arch} ({target_triple})...")
    cmd = ["cargo", "build", "--release"]
    if target_triple:
        cmd.extend(["--target", target_triple])
    
    ret = subprocess.run(cmd)
    if ret.returncode != 0:
        print(f"[!] Build failed for target {arch}!")
        return False

    lib_dir = os.path.join("addon", "globalPlugins", "nvda_fighter", "lib", arch)
    bin_dir = os.path.join("addon", "globalPlugins", "nvda_fighter", "bin", arch)
    os.makedirs(lib_dir, exist_ok=True)
    os.makedirs(bin_dir, exist_ok=True)

    if target_triple:
        target_release = os.path.join("target", target_triple, "release")
    else:
        target_release = os.path.join("target", "release")

    src_dll = os.path.join(target_release, "fighter_core.dll")
    src_exe = os.path.join(target_release, "fighter_daemon.exe")

    if os.path.exists(src_dll):
        shutil.copy2(src_dll, os.path.join(lib_dir, "fighter_core.dll"))
        print(f"    Copied {src_dll} -> {lib_dir}")
    if os.path.exists(src_exe):
        shutil.copy2(src_exe, os.path.join(bin_dir, "fighter_daemon.exe"))
        print(f"    Copied {src_exe} -> {bin_dir}")

    return True

def package_addon(output_path="nvda-fighter.nvda-addon"):
    print(f"[*] Packaging addon into {output_path}...")
    addon_dir = "addon"
    with zipfile.ZipFile(output_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, dirs, files in os.walk(addon_dir):
            for file in files:
                full_path = os.path.join(root, file)
                rel_path = os.path.relpath(full_path, addon_dir)
                zf.write(full_path, rel_path)
    print(f"[+] Add-on successfully created: {output_path} ({os.path.getsize(output_path)} bytes)")

def main():
    parser = argparse.ArgumentParser(description="Build NVDA Fighter Addon")
    parser.add_argument("--all", action="store_true", help="Build both x86 and x64 targets")
    args = parser.parse_args()

    # Build both x86 (standard NVDA Python 32-bit) and x64 by default!
    targets = ["x86", "x64"]
    for t in targets:
        if not build_rust_target(t):
            sys.exit(1)

    package_addon()

if __name__ == "__main__":
    main()
