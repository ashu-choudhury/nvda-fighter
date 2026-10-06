# -*- coding: utf-8 -*-
"""
==============================================================================
⚔️ NVDA FIGHTER: THE SACRED AURA OF IMMORTALITY (Global Plugin) ⚔️
==============================================================================

"By the decree of the Grand Master, NVDA and the Windows Kernel share equal
sovereignty. No mortal terminal dump, no wicked memory leak, and no villainous
buffer overflow shall pierce this sacred citadel."

Class: GRAND GUILD MASTER & SACRED SUMMONER (Level 100)
Guild: The Sacred Order of the Unkillable Screenreader
==============================================================================
"""

import os
import sys
import gc
import ctypes
from ctypes import c_char_p, c_bool, c_uint64, c_int32, POINTER, byref, CFUNCTYPE, c_int
import subprocess
import threading
import time

import globalPluginHandler
import ui
import speech
import api
from scriptHandler import script
from logHandler import log

# Sacred constants
SACRED_PIPE_NAME = r"\\.\pipe\nvda_fighter_sacred_pact"
SACRED_EXIT_HANDSHAKE = b"NVDA_FIGHTER_REST_IN_PEACE_MASTER_FAREWELL\n"

# C-compatible logging callback signature: void(int level, const char* msg)
LOG_CALLBACK_TYPE = CFUNCTYPE(None, c_int, c_char_p)


def _nvda_python_logger(level, msg_ptr):
    """Callback triggered directly from native Rust fighter_core.dll into NVDA log."""
    try:
        msg = msg_ptr.decode("utf-8", errors="replace") if msg_ptr else ""
        if level >= 2:
            log.warning(f"[FIGHTER-NATIVE] {msg}")
        else:
            log.info(f"[FIGHTER-NATIVE] {msg}")
    except Exception:
        pass


# Keep a persistent reference so Python GC doesn't collect the C function pointer
_GLOBAL_NATIVE_LOG_CALLBACK = LOG_CALLBACK_TYPE(_nvda_python_logger)


class FighterNativeBridge:
    """Arcane bridge linking Python to the in-process Rust Shield (fighter_core.dll)."""

    def __init__(self, dll_path: str):
        self.loaded = False
        self._dll = None

        if not os.path.exists(dll_path):
            log.warning(f"[FIGHTER-BRIDGE:WARN] Sacred core not found at {dll_path}")
            return

        try:
            self._dll = ctypes.CDLL(dll_path)

            # 1. Set NVDA Logger
            self._dll.fighter_set_nvda_logger.restype = None
            self._dll.fighter_set_nvda_logger.argtypes = [LOG_CALLBACK_TYPE]

            # 2. Equip Divine Armor
            self._dll.fighter_equip_divine_armor.restype = c_bool
            self._dll.fighter_equip_divine_armor.argtypes = []

            # 3. Intercept Text Dump
            self._dll.fighter_intercept_text_dump.restype = c_char_p
            self._dll.fighter_intercept_text_dump.argtypes = [c_char_p]

            # 4. Free Shield Text
            self._dll.fighter_free_text.restype = None
            self._dll.fighter_free_text.argtypes = [c_char_p]

            # 5. Memory Purge
            self._dll.fighter_purge_memory_leak.restype = c_uint64
            self._dll.fighter_purge_memory_leak.argtypes = []

            # 6. Stats
            self._dll.fighter_get_stats.restype = None
            self._dll.fighter_get_stats.argtypes = [
                POINTER(c_int32),
                POINTER(c_uint64),
                POINTER(c_uint64),
            ]

            # Hook the native logger directly to NVDA
            self._dll.fighter_set_nvda_logger(_GLOBAL_NATIVE_LOG_CALLBACK)

            self.loaded = True
            log.info("[FIGHTER-BRIDGE:SUCCESS] ⚔️ Level 100 Sacred Rust Core loaded into memory!")
        except Exception as e:
            log.error(f"[FIGHTER-BRIDGE:ERROR] Failed to bind to sacred core: {e}")

    def equip_armor(self) -> bool:
        if self.loaded and self._dll:
            return self._dll.fighter_equip_divine_armor()
        return False

    def intercept_text(self, text: str):
        """Passes text to the Rust shield. Returns shielded text or None if safe."""
        if not self.loaded or not self._dll or len(text) <= 10000:
            return None

        try:
            utf8_bytes = text.encode("utf-8", errors="replace")
            res_ptr = self._dll.fighter_intercept_text_dump(utf8_bytes)
            if res_ptr:
                shielded_str = res_ptr.decode("utf-8", errors="replace")
                self._dll.fighter_free_text(res_ptr)
                return shielded_str
        except Exception as e:
            log.error(f"[FIGHTER-BRIDGE:ERROR] Text shield failed: {e}")
        return None

    def purge_memory(self) -> int:
        if self.loaded and self._dll:
            return int(self._dll.fighter_purge_memory_leak())
        return 0

    def get_stats(self):
        if not self.loaded or not self._dll:
            return (9999, 0, 0)
        hp = c_int32(0)
        kills = c_uint64(0)
        bytes_shielded = c_uint64(0)
        self._dll.fighter_get_stats(byref(hp), byref(kills), byref(bytes_shielded))
        return (hp.value, kills.value, bytes_shielded.value)


class GlobalPlugin(globalPluginHandler.GlobalPlugin):
    scriptCategory = "NVDA Fighter"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.plugin_dir = os.path.dirname(os.path.abspath(__file__))
        
        # NVDA runs as 32-bit (x86) on official builds or 64-bit on x64 builds
        arch = "x64" if sys.maxsize > 2**32 else "x86"
        dll_candidates = [
            os.path.join(self.plugin_dir, "lib", arch, "fighter_core.dll"),
            os.path.join(self.plugin_dir, "fighter_core.dll"),
            # Repo targets
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "i686-pc-windows-msvc", "release", "fighter_core.dll")),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "release", "fighter_core.dll")),
        ]
        daemon_candidates = [
            os.path.join(self.plugin_dir, "bin", arch, "fighter_daemon.exe"),
            os.path.join(self.plugin_dir, "fighter_daemon.exe"),
            # Repo targets
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "i686-pc-windows-msvc", "release", "fighter_daemon.exe")),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "release", "fighter_daemon.exe")),
        ]

        dll_path = next((p for p in dll_candidates if os.path.exists(p)), dll_candidates[0])
        self.daemon_path = next((p for p in daemon_candidates if os.path.exists(p)), daemon_candidates[0])

        log.info(f"[NVDA-FIGHTER:INIT] Awakening NVDA Fighter ({arch})... Core path: {dll_path}")

        # 1. Initialize Rust Native Bridge
        self.bridge = FighterNativeBridge(dll_path)
        if self.bridge.loaded:
            self.bridge.equip_armor()

        # 2. Summon the Shadow Beast Daemon (Watchdog & Necromancer)
        self.daemon_proc = None
        self._summon_shadow_beast()

        # 3. Hook into Speech Engine to Defend Against Explosive Text Dumps
        self._original_speak = speech.speak
        self._hook_speech_engine()

        # 4. Start Telepathic Chatter & Mutual Health Patrol Thread (1-second heartbeat)
        self._patrol_active = True
        self._patrol_thread = threading.Thread(target=self._telepathic_chatter_worker, daemon=True)
        self._patrol_thread.start()

        log.info("[NVDA-FIGHTER:READY] 🛡️ NVDA Fighter standing guard! NVDA and Kernel now share equal authority!")

    def _summon_shadow_beast(self):
        """Summons the immortal out-of-process daemon to watch over NVDA."""
        if not os.path.exists(self.daemon_path):
            log.warning(f"[NVDA-FIGHTER:DAEMON] Shadow beast binary not found at {self.daemon_path}")
            return

        try:
            my_pid = str(os.getpid())
            nvda_exe = sys.executable

            DETACHED_PROCESS = 0x00000008
            CREATE_NO_WINDOW = 0x08000000

            self.daemon_proc = subprocess.Popen(
                [self.daemon_path, my_pid, nvda_exe],
                creationflags=DETACHED_PROCESS | CREATE_NO_WINDOW,
                close_fds=True,
            )
            log.info(f"[NVDA-FIGHTER:DAEMON] 🐺 Shadow Beast summoned! Linked to PID {my_pid}.")
        except Exception as e:
            log.error(f"[NVDA-FIGHTER:DAEMON] Failed to summon Shadow Beast: {e}")

    def _hook_speech_engine(self):
        """Intercepts calls to speech.speak to catch text floods before speech chokes."""
        fighter_bridge = self.bridge
        original_speak = self._original_speak

        def fighter_guarded_speak(speechSequence, symbolLevel=None, priority=None):
            modified_seq = []
            shield_triggered = False

            try:
                for item in speechSequence:
                    if isinstance(item, str) and len(item) > 10000:
                        shielded = fighter_bridge.intercept_text(item)
                        if shielded:
                            shield_triggered = True
                            modified_seq.append(shielded)
                            continue
                    modified_seq.append(item)
            except Exception as e:
                log.error(f"[NVDA-FIGHTER:SPEECH] Shield inspection error: {e}")
                modified_seq = speechSequence

            if shield_triggered:
                log.info("[NVDA-FIGHTER:SPEECH] ⚔️ Terminal / Global Dragon Breath neutralised by Shield!")

            return original_speak(modified_seq, symbolLevel=symbolLevel, priority=priority)

        speech.speak = fighter_guarded_speak
        log.info("[NVDA-FIGHTER:SPEECH] 🛡️ Speech Engine guarded by Sacred Text Shield.")

    def _telepathic_chatter_worker(self):
        """
        Two-way telepathic dialogue between Paladin (NVDA) and Beast Daemon (Watchdog).
        Chats every ~1.0 second:
        'How's it going bro?' -> 'All clear on perimeter! Watching the Kernel!'
        Detects internal lag, purges RAM, and broadcasts alerts.
        """
        pulse_count = 0
        while self._patrol_active:
            try:
                time.sleep(1.0)
                if not self._patrol_active:
                    break

                pulse_count += 1
                start_tick = time.perf_counter()

                # Every 30 seconds, perform background GC and memory trim
                if pulse_count % 30 == 0:
                    gc.collect(2)
                    if self.bridge.loaded:
                        self.bridge.purge_memory()

                # Talk to the Shadow Beast via Named Pipe
                if os.path.exists(SACRED_PIPE_NAME):
                    try:
                        # Open pipe in read/write
                        handle = ctypes.windll.kernel32.CreateFileW(
                            SACRED_PIPE_NAME,
                            0xC0000000, # GENERIC_READ | GENERIC_WRITE
                            0,
                            None,
                            3, # OPEN_EXISTING
                            0,
                            None
                        )
                        if handle != -1 and handle != 0:
                            # Send pulse message
                            pulse_msg = f"PULSE: Paladin heartbeat #{pulse_count}\n".encode("utf-8")
                            bytes_written = ctypes.c_ulong(0)
                            ctypes.windll.kernel32.WriteFile(
                                handle,
                                pulse_msg,
                                len(pulse_msg),
                                byref(bytes_written),
                                None
                            )

                            # Read Beast reply
                            buffer = ctypes.create_string_buffer(512)
                            bytes_read = ctypes.c_ulong(0)
                            if ctypes.windll.kernel32.ReadFile(handle, buffer, 512, byref(bytes_read), None):
                                if bytes_read.value > 0:
                                    reply = buffer.value.decode("utf-8", errors="replace").strip()
                                    if pulse_count % 10 == 0: # Log banter occasionally to not spam
                                        log.info(f"[FIGHTER-TELEPATHY] 🐺 {reply}")
                            ctypes.windll.kernel32.CloseHandle(handle)
                    except Exception:
                        pass

                # Lag Detection: If a 1-second sleep took > 2.5 seconds, NVDA's thread lagged!
                elapsed = time.perf_counter() - start_tick
                if elapsed > 2.5:
                    log.warning(f"[NVDA-FIGHTER:LAG-ALERT] ⚠️ Thread stutter detected ({elapsed:.2f}s)! Engaging Alchemical Purge!")
                    gc.collect(2)
                    if self.bridge.loaded:
                        self.bridge.purge_memory()

            except Exception:
                pass

    def terminate(self):
        """Gracefully bid farewell to the Shadow Beast daemon so it does not trigger revival."""
        log.info("[NVDA-FIGHTER:TERMINATE] Initiating Sacred Farewell Handshake...")
        self._patrol_active = False

        if hasattr(self, "_original_speak"):
            speech.speak = self._original_speak

        # Deliver the Sacred Handshake over Named Pipe:
        try:
            if os.path.exists(SACRED_PIPE_NAME):
                handle = ctypes.windll.kernel32.CreateFileW(
                    SACRED_PIPE_NAME,
                    0x40000000, # GENERIC_WRITE
                    0,
                    None,
                    3, # OPEN_EXISTING
                    0,
                    None
                )
                if handle != -1 and handle != 0:
                    written = ctypes.c_ulong(0)
                    ctypes.windll.kernel32.WriteFile(
                        handle,
                        SACRED_EXIT_HANDSHAKE,
                        len(SACRED_EXIT_HANDSHAKE),
                        byref(written),
                        None
                    )
                    ctypes.windll.kernel32.CloseHandle(handle)
                log.info("[NVDA-FIGHTER:HANDSHAKE] Handshake delivered: 'Hey bro, we are exiting. Bye!'")
        except Exception:
            pass

        super().terminate()

    # -----------------------------------------------------------------------
    # HEROIC RPG SCRIPTS / HOTKEYS
    # -----------------------------------------------------------------------

    @script(
        description="Inspect the sacred stats and HP of NVDA Fighter",
        gesture="kb:NVDA+alt+f",
    )
    def script_fighterStats(self, gesture):
        hp, kills, bytes_shielded = self.bridge.get_stats()
        kb_shielded = bytes_shielded // 1024
        msg = (
            f"NVDA Fighter Level 100 Status: HP is {hp} out of 9999. "
            f"Villainous dumps vanquished: {kills}. "
            f"Total bytes shielded: {kb_shielded} kilobytes. "
            f"Status: Divine Titan Armor Active. NVDA and Kernel share equal authority!"
        )
        ui.message(msg)

    @script(
        description="Cast Alchemical Purge: Force immediate garbage collection and RAM trim",
        gesture="kb:NVDA+alt+shift+f",
    )
    def script_alchemicalPurge(self, gesture):
        gc.collect(2)
        freed = self.bridge.purge_memory()
        mb = freed // (1024 * 1024)
        ui.message(f"Alchemical Purge Complete! Banished {mb} Megabytes of dead memory. System refreshed!")
