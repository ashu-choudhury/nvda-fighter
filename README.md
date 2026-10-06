# ⚔️ NVDA Fighter: The Immortal RPG Guardian Shield & Necromancer 🛡️

> *"By the decree of the Grand Master, NVDA and the Windows Kernel share equal sovereignty. No mortal terminal dump, no wicked memory leak, and no villainous buffer overflow shall pierce this sacred citadel."*

**NVDA Fighter** is a combat-grade, unkillable guardian shield for NVDA written in native **Rust** and **Python**. It treats screenreader crashes as an affront to honor, defending NVDA at all costs using low-level Win32 operating system mechanics, alchemical memory purges, high-speed text flood dampers, and an immortal out-of-process Shadow Beast daemon.

---

## 🎮 The RPG Classes & Pillars

### 1. 🛡️ The Shield-Bearer (`fighter_core.dll` - Native Rust)
* **Divine Win32 Armor:** Elevates NVDA to `HIGH_PRIORITY_CLASS`, locks priority to prevent Windows dynamic de-boost, and silences GP Fault modal crash dialogs (`SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX`).
* **Dragon-Breath Text Shield:** Intercepts massive text dumps (e.g., terminal output explosions > 10,000 characters). Slashes the hazardous middle, shields NVDA's speech buffer from overflowing, and delivers the clean 500-character tail remnant with battle statistics.
* **Alchemical Memory Purge:** Trims working sets (`EmptyWorkingSet`) and forces Python Gen-2 garbage collection whenever memory pressure rises.

### 2. 🐺 The Shadow Beast & Necromancer (`fighter_daemon.exe` - Background Watchdog)
* **The Sacred Soul Pact:** Runs as an ultra-featherweight background predator (< 1.5 MB RAM) with an open synchronization handle to NVDA.
* **The Sacred Exit Handshake:** Listens on a named pipe (`\\.\pipe\nvda_fighter_sacred_pact`). When NVDA legitimately closes, it whispers:  
  `"Hey bro, we are exiting. You also exit."` — allowing the daemon to sleep peacefully.
* **Level 100 True Resurrection:** If NVDA vanishes *without* saying goodbye (crash, termination, taskkill, audio driver stall):
  * The Beast roars in the logs:  
    `"OH MY GOD! NVDA VANISHED WITHOUT SAYING GOODBYE! ENGAGING 100x SACRED REVIVAL RITUAL!"`
  * Relaunches NVDA instantly (`nvda.exe -r`) in less than 200ms before you even notice speech was gone!

---

## ⌨️ Heroic Hotkeys

* **`NVDA + Alt + F`**: Inspect NVDA Fighter RPG stats (Current HP, Villains Vanquished, Total Bytes Shielded, and Divine Armor Status).
* **`NVDA + Alt + Shift + F`**: Cast **Alchemical Purge** (forces instant Python GC + flushes working set RAM back to Windows).

---

## 🛠️ Building & Packaging

### Prerequisites
* Rust toolchain (`cargo`, `rustc`)
* Python 3.10+

### Build Host Add-on
```powershell
python build.py
```

### Multi-Architecture Build (x64, x86, ARM64)
```powershell
python build.py --all
```

The resulting package is generated as `nvda-fighter.nvda-addon`.

---

## 📜 License
Licensed under the GNU General Public License v2 (GPL-2.0).
